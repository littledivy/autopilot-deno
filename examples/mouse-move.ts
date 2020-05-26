import AutoPilot from 'https://raw.githubusercontent.com/divy-work/autopilot-deno/master/mod.ts';
var pilot = new AutoPilot();
var screenSize = pilot.screenSize();
var widthUnit: number = screenSize.height / 8;
console.log(widthUnit);
setInterval(function () {
	var position = pilot.mousePosition();
	if (position.x > widthUnit * 2) {
		pilot.moveMouse(widthUnit,position.y)
	}else{
		pilot.moveMouse(widthUnit*3,position.y)
	}
}, 200);
