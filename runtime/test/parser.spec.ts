import { depsParser } from "../rpc/parser";

describe("deps parser test", () => {
  const modules = [
    {
      name: "A",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: []
    },
    {
      name: "B",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["A"]
    },
    {
      name: "C",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["A", "B", "D"]
    },
    {
      name: "D",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["F"]
    },
    {
      name: "E",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["D", "C"]
    },
    {
      name: "F",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: []
    },
    {
      name: "G",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["H"]
    },
    {
      name: "H",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["G"]
    },
  ];

  const expectRes = [
    {
      name: "A",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: []
    },
    {
      name: "F",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: []
    },
    {
      name: "B",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["A"]
    },
    {
      name: "D",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["F"]
    },
    {
      name: "C",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["A", "B", "D"]
    },
    {
      name: "E",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["D", "C"]
    },
    {
      name: "G",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["H"]
    },
    {
      name: "H",
      path: "target/wasm32-unknown-unknown/release/actor.wasm",
      deps: ["G"]
    },
  ]

  it("should return expectRes", () => {
    const result = depsParser(modules);
    expect(result).toStrictEqual(expectRes);
  })
});
