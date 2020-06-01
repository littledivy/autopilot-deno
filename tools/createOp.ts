type OPType = "no-return" | "no-return-serde" | "return" | "return-serde" | "all";

const baseTemplate = (name: string, code: string) => `
fn op_${name}(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {
        let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
        // TODO: stuff below
    ${code}
        Op::Sync(result_box)
}
`;

export default function createOP(name: string = Deno.args[0], type: string = Deno.args[1]) {
  switch (type) {
    case "no-return":
      return baseTemplate(name, `
      let result = b"true";
      let result_box: Buf = Box::new(*result);
      `)
      break;
    case "no-return-serde":
      return baseTemplate(name, `
        #[derive(Deserialize)]
        struct SomeSerde {
            x: f64,
            y: f64,
        }
        let params: SomeSerde = serde_json::from_slice(data).unwrap();
        let result = b"true";
        let result_box: Buf = Box::new(*result);
      `)
      break;
    case "return":
      return baseTemplate(name, `
      let result = b"true";
      let result_box: Buf = Box::new(*result);
      `)
      break;
    case "return-serde":
      return baseTemplate(name, `
      #[derive(Serialize)]
      struct SomeResponse {
        thing: str,
      }
      let response = SomeResponse {
        thing: "a"
      };
      let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
      `)
      break;
    case "all":
      return baseTemplate(name, `
        #[derive(Deserialize)]
        struct SomeSerde {
            x: f64,
            y: f64,
        }
        #[derive(Serialize)]
        struct SomeResponse {
          thing: str,
        }
        let params: SomeSerde = serde_json::from_slice(data).unwrap();
        let response = SomeResponse {
          thing: "a"
        };
        let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
      `)
      break;
  }
}

console.log(createOP());
