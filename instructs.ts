const text = await Deno.readTextFile("instructs.json")

const ops = JSON.parse(text);

const res = new Map()

const address_mode = {
  "implied": "Impl",
  "accum": "Accumulator",
  "imm": "Immediate",
  "relative": "Relative",
  "zp": "ZeroPage",
  "zpx": "ZeroPageX",
  "zpy": "ZeroPageY",
  "abs": "Absolute",
  "absx": "AbsoluteX",
  "absy": "AbsoluteY",
  "ind": "Indirect",
  "indx": "IndirectX",
  "indy": "IndirectY",
}
const add_i = {
  "Impl": 0,
  "Accumulator": 1,
  "Immediate": 2,
  "Relative": 3,
  "ZeroPage": 4,
  "ZeroPageX": 5,
  "ZeroPageY": 6,
  "Absolute": 7,
  "AbsoluteX": 8,
  "AbsoluteY": 9,
  "Indirect": 10,
  "IndirectX": 11,
  "IndirectY": 12,
}
let index = -1;
for (let op of ops) {
  index++;
  if (op.name == "ILLEGAL" || op.address_mode == "ILLEGAL") {
    continue;
  }
  const add = address_mode[op.address_mode];
  if (add == null) {
    throw new Error("Invalid OpCode: " + op.address_mode);
  }
  // if (op.machine_cycles === 2 && ['Immediate', 'Accumulator', 'Impl'].includes(add)) {
  // }
  if (op.machine_cycles === 2) {
    console.log(`0x${index.toString(16).toUpperCase()} => Some((Instruct::${op.name}, AddressType::${add}), ),`)
  }
  res.set(op.name, [...res.get(op.name) ?? [], [add, index]]);
};
// let all_add = [...res.entries()].sort((a, b) => a[0] < b[0] ? -1 : 1);
//
// let result = "";
// for (let [key, codes] of all_add) {
//   let str = `Instruct::${key} => match addr {\n`;
//   codes = codes.sort((a, b) => add_i[a[0]] < add_i[b[0]] ? -1 : 1);
//   for (let [add, op_code] of codes) {
//     str += `AddressType::${add} => Some(0x${op_code.toString(16).toUpperCase()}),\n`
//   }
//   str += "_ => None,\n"
//   str += "},\n"
//   result += str;
// }
//
// console.log(result);
