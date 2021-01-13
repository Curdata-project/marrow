type Modules = Module[];

type Module = {
  name: string;
  path: string;
  deps: string[];
};

type Method = {
  name: string;
  module: string;
  ty: "async" | "sync";
  arguments: Arg[];
  return: Return;
};

type Arg = {
  name: string;
  type: ValueType;
};

type Return = {
  type: ValueType;
};

type ValueType = "u32" | "i32" | "u64" | "i64" | "isize" | "usize" | "bytes" | "null";

type ParseModuleList = ParseModule[];

type ParseModule = {
  name: string;
  instance: WebAssembly.Instance;
};

type ParseMethodList = Method[];

type RequestCache = number[];

type RPCRequest = {
  jsonrpc: string;
  module: string;
  name: string;
  index: number;
  args: any[];
  type: string;
};

type RPCResponse = {
  jsonrpc: string;
  index: number;
  code: number;
  result: any;
};



declare namespace WebAssembly {
  export class Module {
    static customSections(module: Module, sectionName: string): ArrayBuffer;
    static exports(module: Module): object;
    static imports(module: Module): object;
    toString(): 'WebAssembly.Module';
  }

  export class Global {
    value: any;
    constructor(
      descriptor: { value: 'i32' | 'i64' | 'f32' | 'f64'; mutable?: boolean },
      value: any
    );
    toString(): 'WebAssembly.Global';
    valueOf(): any;
  }

  export class Instance {
    readonly exports: { [K: string]: Function };
    constructor(module: Module, importObject?: object);
    grow(pages: number): number;
  }

  export class Memory {
    buffer: ArrayBuffer;
    constructor(memoryDescriptor: { initial: number; maximum?: number });
  }

  export class Table {
    length: number;
    constructor(tableDescriptor: {
      element: string;
      initial: number;
      maximum?: number;
    });
    get(index: number): Function;
    grow(elements: number): number;
    set(index: number, value: Function): void;
  }

  export class CompileError extends Error {}

  export class LinkError extends Error {}

  export class RuntimeError extends Error {}

  export interface ResultObject {
    module: Module;
    instance: Instance;
  }

  export function instantiate(
    source: Buffer,
    importObject?: object
  ): Promise<ResultObject>;
}