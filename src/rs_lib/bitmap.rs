// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
//! This module defines the struct `Bitmap` for accessing bitmaps and
//! searching for bitmaps on-screen.
//!
//! It also defines functions for taking screenshots of the screen.
extern crate image;

use geometry::{Point, Rect, Size};
use image::{DynamicImage, GenericImage, GenericImageView, ImageError, ImageResult, Pixel, Rgba};
use screen;
use std;

#[cfg(target_os = "macos")]
use core_graphics::geometry::CGRect;
#[cfg(target_os = "macos")]
use core_graphics::image::CGImage;
#[cfg(target_os = "macos")]
use libc;

#[cfg(target_os = "linux")]
use internal;
#[cfg(not(target_os = "macos"))]
use scopeguard::guard;
#[cfg(target_os = "linux")]
use x11;

#[derive(Clone)]
pub struct Bitmap {
    pub image: DynamicImage,

    /// Size of the bitmap in points.
    pub size: Size,

    pub scale: f64,
}

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Bitmap {{ size: {}, scale: {} }}", self.size, self.scale)
    }
}

impl std::cmp::PartialEq for Bitmap {
    fn eq(&self, other: &Bitmap) -> bool {
        self.bitmap_eq(other, None)
    }
}

impl std::hash::Hash for Bitmap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(rgb_image) = self.image.as_rgba8() {
            rgb_image.hash(state);
        }
        self.size.hash(state);
        (self.scale as i64).hash(state);
    }
}

impl Bitmap {
    #[inline]
    /// Creates a bitmap from the given `DynamicImage`, and scale if given
    /// (defaults to 1).
    pub fn new(image: DynamicImage, scale: Option<f64>) -> Bitmap {
        let scale: f64 = scale.unwrap_or(1.0);
        Bitmap {
            size: Size::new(
                f64::from(image.width()) / scale,
                f64::from(image.height()) / scale,
            ),
            image,
            scale,
        }
    }

    #[inline]
    /// Returns bounds of bitmap as a rect, with an origin of zero.
    pub fn bounds(&self) -> Rect {
        Rect::new(Point::ZERO, self.size)
    }

    /// Copies image to pasteboard. Currently only supported on macOS.
    pub fn copy_to_pasteboard(&self) -> ImageResult<()> {
        self.system_copy_to_pasteboard()
    }

    /// Returns new Bitmap created from a portion of another.
    pub fn cropped(&mut self, rect: Rect) -> ImageResult<Bitmap> {
        if !self.bounds().is_rect_visible(rect) {
            Err(ImageError::DimensionError)
        } else {
            let rect = rect.scaled(self.scale).round();
            let cropped_image = self.image.crop(
                rect.origin.x as u32,
                rect.origin.y as u32,
                rect.size.width as u32,
                rect.size.height as u32,
            );
            Ok(Bitmap::new(cropped_image, Some(self.scale)))
        }
    }

    // Returns color of underlying image at the given point.
    pub fn get_pixel(&self, point: Point) -> Rgba<u8> {
        let point = point.scaled(self.multiplier()).round();
        self.image.get_pixel(point.x as u32, point.y as u32)
    }

    /// Returns true if bitmap is equal to needle with the given tolerance.
    pub fn bitmap_eq(&self, needle: &Bitmap, tolerance: Option<f64>) -> bool {
        self.size == needle.size
            && self.scale == needle.scale
            && self.is_needle_at(Point::ZERO, needle, tolerance)
    }

    /// Attempts to find `color` inside `rect` in `bmp` from the given
    /// `start_point`. Returns coordinates if found, or `None` if not. If
    /// `rect` is `None`, `bmp.bounds()` is used instead. If `start_point` is
    /// `None`, the origin of `rect` is used.
    ///
    /// Tolerance is defined as a float in the range from 0 to 1, where 0 is
    /// an exact match and 1 matches anything.
    pub fn find_color(
        &self,
        needle: Rgba<u8>,
        tolerance: Option<f64>,
        rect: Option<Rect>,
        start_point: Option<Point>,
    ) -> Option<Point> {
        let tolerance = tolerance.unwrap_or(0.0);
        self.find(rect, start_point, |point| {
            colors_match(needle, self.get_pixel(point), tolerance)
        })
    }

    /// Returns list of all coordinates inside `rect` in `bmp` matching
    /// `color` from the given `start_point`. If `rect` is `None`,
    /// `bmp.bounds()` is used instead. If `start_point` is `None`, the origin
    /// of `rect` is used.
    pub fn find_every_color(
        &self,
        needle: Rgba<u8>,
        tolerance: Option<f64>,
        rect: Option<Rect>,
        start_point: Option<Point>,
    ) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        {
            let tolerance = tolerance.unwrap_or(0.0);
            let mut matched = |point| {
                points.push(point);
            };
            self.find_all(
                rect,
                start_point,
                &(|point| colors_match(needle, self.get_pixel(point), tolerance)),
                &mut matched,
            );
        }
        points
    }

    /// Returns count of color in bitmap. Functionally equivalent to:
    /// ```rust,ignore
    /// find_every_color(color, tolerance, rect, start_point).count()
    /// ```
    pub fn count_of_color(
        &self,
        needle: Rgba<u8>,
        tolerance: Option<f64>,
        rect: Option<Rect>,
        start_point: Option<Point>,
    ) -> u64 {
        let mut count: u64 = 0;
        {
            let tolerance = tolerance.unwrap_or(0.0);
            let mut matched = |_| {
                count += 1;
            };
            self.find_all(
                rect,
                start_point,
                &(|point| colors_match(needle, self.get_pixel(point), tolerance)),
                &mut matched,
            );
        }
        count
    }

    /// Attempts to find `needle` inside `rect` in `bmp` from the given
    /// `start_point`. Returns coordinates if found, or `None` if not. If
    /// `rect` is `None`, `bmp.bounds()` is used instead. If `start_point` is
    /// `None`, the origin of `rect` is used.
    ///
    /// Tolerance is defined as a float in the range from 0 to 1, where 0 is
    /// an exact match and 1 matches anything.
    pub fn find_bitmap(
        &self,
        needle: &Bitmap,
        tolerance: Option<f64>,
        rect: Option<Rect>,
        start_point: Option<Point>,
    ) -> Option<Point> {
        if self.is_needle_oversized(needle) {
            return None;
        }

        self.find(rect, start_point, |pt| {
            self.is_needle_at(pt, needle, tolerance)
        })
    }

    /// Returns list of all coordinates inside `rect` in `bmp` matching
    /// `needle` from the given `start_point`. If `rect` is `None`,
    /// `bmp.bounds` is used instead. If `start_point` is `None`, the origin
    /// of `rect` is used.
    pub fn find_every_bitmap(
        &self,
        needle: &Bitmap,
        tolerance: Option<f64>,
        rect: Option<Rect>,
        start_point: Option<Point>,
    ) -> Vec<Point> {
        if self.is_needle_oversized(needle) {
            return Vec::new();
        }

        let mut points: Vec<Point> = Vec::new();
        {
            let mut matched = |point| {
                points.push(point);
            };
            self.find_all(
                rect,
                start_point,
                &(|pt| self.is_needle_at(pt, needle, tolerance)),
                &mut matched,
            );
        }
        points
    }

    /// Returns count of occurrences of `needle` in `bmp`. Functionally equivalent to:
    ///
    /// ```rust,ignore
    /// find_every_bitmap(color, tolerance, rect, start_point).count()
    /// ```
    pub fn count_of_bitmap(
        &self,
        needle: &Bitmap,
        tolerance: Option<f64>,
        rect: Option<Rect>,
        start_point: Option<Point>,
    ) -> u64 {
        if self.is_needle_oversized(needle) {
            return 0;
        }

        let mut count: u64 = 0;
        {
            let mut matched = |_| {
                count += 1;
            };
            self.find_all(
                rect,
                start_point,
                &(|pt| self.is_needle_at(pt, needle, tolerance)),
                &mut matched,
            );
        }
        count
    }

    #[inline]
    fn multiplier(&self) -> f64 {
        1.0 / self.scale
    }

    #[inline]
    fn is_needle_oversized(&self, needle: &Bitmap) -> bool {
        needle.scale > self.scale
            || needle.bounds().size.width > self.bounds().size.width
            || needle.bounds().size.height > self.bounds().size.height
    }

    fn is_needle_at(&self, pt: Point, needle: &Bitmap, tolerance: Option<f64>) -> bool {
        let bounds = needle.bounds();
        for x in bounds.origin.x as u64..bounds.max_x() as u64 {
            for y in bounds.origin.y as u64..bounds.max_y() as u64 {
                let needle_point = Point::new(x as f64, y as f64);
                let haystack_point = Point::new(pt.x + needle_point.x, pt.y + needle_point.y);
                if !self.bounds().is_point_visible(haystack_point) {
                    return false;
                }

                let c1 = needle.get_pixel(needle_point);
                let c2 = self.get_pixel(haystack_point);
                if !colors_match(c1, c2, tolerance.unwrap_or(0.0f64)) {
                    return false;
                }
            }
        }

        true
    }

    fn find<F: Fn(Point) -> bool>(
        &self,
        rect: Option<Rect>,
        start_point: Option<Point>,
        predicate: F,
    ) -> Option<Point> {
        let rect = rect.unwrap_or_else(|| self.bounds());
        let start_point = start_point.unwrap_or(self.bounds().origin);
        if !self.bounds().is_rect_visible(rect) {
            panic!(
                "invalid rect: {} outside of image bounds ({})",
                rect,
                self.bounds()
            );
        }
        if !self.bounds().is_point_visible(start_point) {
            panic!(
                "invalid start point: {} outside of image bounds ({})",
                start_point,
                self.bounds()
            );
        }

        // TODO: Switch the Boyer-Moore algorithm for image search or use this instead
        // http://bit.ly/1EIEIfr.
        let start_point = start_point.scaled(self.multiplier()).round();
        let rect = rect.scaled(self.multiplier()).round();
        let mut start_y = start_point.y;
        for x in start_point.x as u64..rect.max_x() as u64 {
            for y in start_y as u64..rect.max_y() as u64 {
                let point = Point::new(x as f64, y as f64);
                if predicate(point) {
                    return Some(point.scaled(self.scale).round());
                }
            }
            start_y = rect.origin.y;
        }

        None
    }

    fn find_all<'a>(
        &self,
        rect: Option<Rect>,
        start_point: Option<Point>,
        predicate: &'a dyn Fn(Point) -> bool,
        matched: &'a mut dyn FnMut(Point) -> (),
    ) {
        let rect = rect.unwrap_or_else(|| self.bounds());
        let mut start_point = start_point.unwrap_or(self.bounds().origin);
        loop {
            if let Some(point) = self.find(Some(rect), Some(start_point), predicate) {
                matched(point);
                if let Some(next_point) = rect.iter_point(point) {
                    start_point = next_point;
                    continue;
                }
            }

            break;
        }
    }

    #[cfg(target_os = "macos")]
    fn system_copy_to_pasteboard(&self) -> ImageResult<()> {
        use cocoa::appkit::{NSImage, NSPasteboard};
        use cocoa::base::nil;
        use cocoa::foundation::{NSArray, NSData};
        use image::ImageFormat;

        let mut buffer: Vec<u8> = Vec::new();
        self.image.write_to(&mut buffer, ImageFormat::PNG)?;
        unsafe {
            let data = NSData::dataWithBytes_length_(
                nil,
                buffer.as_ptr() as *const std::os::raw::c_void,
                buffer.len() as u64,
            );
            let image = NSImage::initWithData_(NSImage::alloc(nil), data);
            let objects = NSArray::arrayWithObject(nil, image);
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            pasteboard.clearContents();
            pasteboard.writeObjects(objects);
        }
        Ok(())
    }

    #[cfg(windows)]
    fn system_copy_to_pasteboard(&self) -> ImageResult<()> {
        panic!("Unsupported OS");
    }

    #[cfg(target_os = "linux")]
    fn system_copy_to_pasteboard(&self) -> ImageResult<()> {
        panic!("Unsupported OS");
    }
}

/// Returns true if the given two colors are sufficiently similar.
///
/// Tolerance is defined as a double in the range from 0 to 1, where 0 is an
/// exact match and 1 matches anything.
#[inline]
fn colors_match(c1: Rgba<u8>, c2: Rgba<u8>, tolerance: f64) -> bool {
    assert!(
        tolerance >= 0.0 && tolerance <= 1.0,
        "Tolerance must be between 0 and 1."
    );
    if tolerance == 0.0 {
        return c1 == c2;
    }

    let (r1, g1, b1, _) = c1.channels4();
    let (r2, g2, b2, _) = c2.channels4();
    let d1: f64 = (f64::from(r1) - f64::from(r2)).abs();
    let d2: f64 = (f64::from(g1) - f64::from(g2)).abs();
    let d3: f64 = (f64::from(b1) - f64::from(b2)).abs();
    (d1 * d1 + d2 * d2 + d3 * d3).sqrt() <= tolerance * MAX_TOLERANCE_DELTA
}

const MAX_TOLERANCE_DELTA: f64 = 441.672_955_930_1; // => (3.0f64 * 255.0f64 * 255.0f64).sqrt();

/// Returns a screengrab of the entire main display.
pub fn capture_screen() -> ImageResult<Bitmap> {
    capture_screen_portion(Rect::new(Point::ZERO, screen::size()))
}

/// Returns a screengrab of the given portion of the main display.
pub fn capture_screen_portion(rect: Rect) -> ImageResult<Bitmap> {
    if !screen::is_rect_visible(rect) {
        Err(ImageError::DimensionError)
    } else {
        system_capture_screen_portion(rect)
    }
}

#[cfg(target_os = "macos")]
fn system_capture_screen_portion(rect: Rect) -> ImageResult<Bitmap> {
    use core_graphics::display::CGDisplay;
    if let Some(image) = CGDisplay::screenshot(CGRect::from(rect), 0, 0, 0) {
        macos_load_cgimage(&image)
    } else {
        Err(ImageError::NotEnoughData)
    }
}

#[cfg(windows)]
fn system_capture_screen_portion(rect: Rect) -> ImageResult<Bitmap> {
    use winapi::ctypes::c_void;
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::windef::HGDIOBJ;
    use winapi::um::wingdi::{
        BitBlt, CreateCompatibleDC, CreateDIBSection, SelectObject, DIB_RGB_COLORS, SRCCOPY,
    };
    use winapi::um::wingdi::{DeleteDC, DeleteObject};
    use winapi::um::wingdi::{BITMAPINFO, BITMAPINFOHEADER, BI_RGB};
    use winapi::um::winuser::{GetDC, ReleaseDC};

    let rect = rect.scaled(screen::scale());
    let bytes_per_pixel: usize = 4;
    let bytewidth = rect.size.width as usize * bytes_per_pixel;
    let bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: rect.size.width as i32,
            biHeight: -rect.size.height as i32, // Indicates non-cartesian coordinates.
            biPlanes: 1,
            biBitCount: (bytes_per_pixel * 8) as u16,
            biCompression: BI_RGB,
            biSizeImage: (rect.size.width * rect.size.height) as DWORD * bytes_per_pixel as DWORD,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: unsafe { std::mem::zeroed() },
    };

    // Copy entire screen.
    let screen = unsafe {
        guard(GetDC(std::ptr::null_mut()), |s| {
            ReleaseDC(std::ptr::null_mut(), s);
        })
    };
    if screen.is_null() {
        return Err(ImageError::NotEnoughData);
    }

    // Get screen data in display device context.
    let mut data: *mut c_void = std::ptr::null_mut();
    let dib = unsafe {
        guard(
            CreateDIBSection(
                *screen,
                &bitmap_info,
                DIB_RGB_COLORS,
                &mut data,
                std::ptr::null_mut(),
                0,
            ),
            |d| {
                DeleteObject(d as HGDIOBJ);
            },
        )
    };

    // Copy data into bitmap struct.
    let screen_mem = unsafe {
        guard(CreateCompatibleDC(*screen), |s| {
            DeleteDC(s);
        })
    };
    unsafe {
        if screen_mem.is_null()
            || SelectObject(*screen_mem, *dib as HGDIOBJ).is_null()
            || BitBlt(
                *screen_mem,
                0,
                0,
                rect.size.width as i32,
                rect.size.height as i32,
                *screen,
                rect.origin.x as i32,
                rect.origin.y as i32,
                SRCCOPY,
            ) == 0
        {
            return Err(ImageError::NotEnoughData);
        }
    };

    let buflen: usize = rect.size.height as usize * bytewidth;
    let buffer: &[u8] = unsafe { std::slice::from_raw_parts(data as *mut u8, buflen) };
    let mut img = DynamicImage::new_rgb8(rect.size.width as u32, rect.size.height as u32);
    for x in 0..rect.size.width as usize {
        for y in 0..rect.size.height as usize {
            let offset: usize =
                bytewidth as usize * y as usize + bytes_per_pixel as usize * x as usize;
            let (b, g, r) = (buffer[offset], buffer[offset + 1], buffer[offset + 2]);
            img.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
        }
    }

    Ok(Bitmap::new(img, Some(screen::scale())))
}

#[cfg(target_os = "linux")]
fn system_capture_screen_portion(rect: Rect) -> ImageResult<Bitmap> {
    internal::X_MAIN_DISPLAY.with(|display| {
        let scaled_rect = rect.scaled(screen::scale());
        let root_window = unsafe {
            guard(x11::xlib::XDefaultRootWindow(display.as_ptr()), |w| {
                x11::xlib::XDestroyWindow(display.as_ptr(), w);
            })
        };
        let image_ptr = unsafe {
            guard(
                x11::xlib::XGetImage(
                    display.as_ptr(),
                    *root_window,
                    scaled_rect.origin.x as i32,
                    scaled_rect.origin.y as i32,
                    scaled_rect.size.width as u32,
                    scaled_rect.size.height as u32,
                    x11::xlib::XAllPlanes(),
                    x11::xlib::ZPixmap,
                ),
                |i| {
                    x11::xlib::XDestroyImage(i);
                },
            )
        };
        if image_ptr.is_null() {
            return Err(ImageError::NotEnoughData);
        }
        let image = unsafe { **image_ptr };
        let bytes_per_pixel = image.bits_per_pixel / 8;
        let buflen: usize = image.width as usize * image.height as usize * bytes_per_pixel as usize;
        let buffer: &[u8] = unsafe { std::slice::from_raw_parts(image.data as *mut u8, buflen) };
        let mut img = DynamicImage::new_rgb8(image.width as u32, image.height as u32);
        for x in 0..image.width {
            for y in 0..image.height {
                let offset: usize = image.bytes_per_line as usize * y as usize
                    + bytes_per_pixel as usize * x as usize;
                let (b, g, r) = (buffer[offset], buffer[offset + 1], buffer[offset + 2]);
                img.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
            }
        }
        let bmp = Bitmap::new(img, Some(screen::scale()));
        Ok(bmp)
    })
}

#[cfg(target_os = "macos")]
fn macos_load_cgimage(image: &CGImage) -> ImageResult<Bitmap> {
    use core_graphics::base::CGFloat;
    use core_graphics::context::CGContext;
    use core_graphics::geometry::{CGSize, CG_ZERO_POINT};
    use core_graphics::image::{CGImageAlphaInfo, CGImageByteOrderInfo};

    let width: libc::size_t = image.width();
    let height: libc::size_t = image.height();
    let bits_per_component: libc::size_t = image.bits_per_component();
    let bytes_per_pixel: libc::size_t = image.bits_per_pixel() / 8;
    let bytes_per_row: libc::size_t = image.bytes_per_row();
    let space = image.color_space();
    let flags: u32 = CGImageByteOrderInfo::CGImageByteOrder32Big as u32
        | CGImageAlphaInfo::CGImageAlphaNoneSkipLast as u32;
    let mut context = CGContext::create_bitmap_context(
        None,
        width,
        height,
        bits_per_component,
        bytes_per_row,
        &space,
        flags,
    );
    let rect = CGRect {
        origin: CG_ZERO_POINT,
        size: CGSize::new(width as CGFloat, height as CGFloat),
    };

    context.draw_image(rect, &image);

    let buffer: &[u8] = context.data();
    let mut dynimage = DynamicImage::new_rgb8(width as u32, height as u32);
    for x in 0..width {
        for y in 0..height {
            let offset = bytes_per_row * y + bytes_per_pixel * x;
            let (r, g, b) = (buffer[offset], buffer[offset + 1], buffer[offset + 2]);
            dynimage.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
        }
    }
    let bmp = Bitmap::new(dynimage, Some(screen::scale()));
    Ok(bmp)
}

#[cfg(test)]
mod tests {
    use bitmap::{capture_screen, capture_screen_portion, colors_match, Bitmap};
    use geometry::{Point, Rect, Size};
    use image::{DynamicImage, Rgba, RgbaImage};
    use image::{GenericImage, GenericImageView};
    use quickcheck::{Arbitrary, Gen, TestResult};
    use rand::prelude::SliceRandom;
    use rand::{thread_rng, Rng};

    impl Arbitrary for Bitmap {
        fn arbitrary<G: Gen>(g: &mut G) -> Bitmap {
            let xs = Vec::<u8>::arbitrary(g);
            let scale: f64 = [1.0, 2.0].choose(g).unwrap().clone();
            let width: f64 = (xs.len() as f64 / 4.0).floor().sqrt();
            let image = RgbaImage::from_raw(width as u32, width as u32, xs).unwrap();
            let dynimage = DynamicImage::ImageRgba8(image);
            return Bitmap::new(dynimage, Some(scale));
        }
    }

    #[test]
    #[should_panic]
    fn test_colors_match_low_tolerance() {
        colors_match(Rgba([0, 0, 0, 255]), Rgba([0, 0, 0, 255]), -0.1);
    }

    #[test]
    #[should_panic]
    fn test_colors_match_high_tolerance() {
        colors_match(Rgba([0, 0, 0, 255]), Rgba([0, 0, 0, 255]), 1.1);
    }

    #[test]
    fn test_capture_screen_portion() {
        let rect = Rect::new(Point::new(100.0, 100.0), Size::new(100.0, 100.0));
        let portion: Bitmap = capture_screen_portion(rect).unwrap();
        let mut uncropped: Bitmap = capture_screen().unwrap();
        let cropped: Bitmap = uncropped.cropped(rect).unwrap();
        assert_eq!(portion, cropped)
    }

    quickcheck! {
        fn finds_cropped_bitmap(haystack: Bitmap) -> TestResult {
            if haystack.size.width < 2.0 {
                return TestResult::discard();
            }

            let mut rng = thread_rng();
            let crop_scale: f64 = rng.gen_range(0.1, 1.0);
            let offset_percentage: f64 = rng.gen_range(0.0, 1.0);
            let mut cropped_width = (haystack.size.width * crop_scale).round();
            let mut cropped_height = (haystack.size.height * crop_scale).round();
            if cropped_width < 1.0 * haystack.scale {
                cropped_width = 1.0 * haystack.scale;
            }
            if cropped_height < 1.0 * haystack.scale {
                cropped_height = 1.0 * haystack.scale;
            }
            let offset_pt = Point::new(
                (haystack.size.width - cropped_width) * offset_percentage,
                (haystack.size.height - cropped_height) * offset_percentage
            ).round();
            let needle = haystack.clone().cropped(Rect::new(
                offset_pt,
                Size::new(cropped_width, cropped_height)
            )).unwrap();
            let pt_a = haystack.find_bitmap(&needle, None, None, None);
            let pt_b = haystack.find_bitmap(&needle, None, None, Some(offset_pt));
            return TestResult::from_bool(pt_a.is_some() &&
                                         pt_b.is_some() &&
                                         pt_b.unwrap() == offset_pt);
        }
    }

    quickcheck! {
        fn skips_inverted_bitmap(haystack: Bitmap) -> TestResult {
            if haystack.size.width == 0.0 {
                return TestResult::discard();
            }

            let mut inverted = haystack.image.clone();
            inverted.invert();
            let needle = Bitmap::new(inverted, None);
            let pt = haystack.find_bitmap(&needle, None, None, None);
            return TestResult::from_bool(pt.is_none());
        }
    }

    quickcheck! {
        fn count_of_tiled_bitmap(tile: Bitmap) -> TestResult {
            if tile.size.width <= 2.0 {
                return TestResult::discard();
            }
            let mut haystack_img = DynamicImage::new_rgba8(
                tile.image.width() as u32 * 2 as u32 + 1,
                tile.image.height() as u32 * 2 as u32 + 1
            );
            for x in 0..tile.image.width() as u32 * 2 as u32 {
                for y in 0..tile.image.height() as u32 * 2 as u32 {
                    let tile_x = x % tile.image.width() as u32;
                    let tile_y = y % tile.image.height() as u32;
                    haystack_img.put_pixel(
                        x as u32,
                        y as u32,
                        tile.image.get_pixel(tile_x, tile_y)
                    );
                }
            }

            let haystack = Bitmap::new(haystack_img, Some(tile.scale));
            return TestResult::from_bool(haystack.count_of_bitmap(&tile, None, None, None) >= 4);
        }
    }
}
