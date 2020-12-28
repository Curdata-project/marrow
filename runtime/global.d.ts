type Modules = Module[];

type Module = {
  name: string;
  path: string;
  deps: string[];
  expose: Expose[];
};

type Expose = {
  type: string;
  name: string;
  args: Arg[];
  return: Return;
};

type Arg = {
  name: string;
  type: string;
  attr?: {
    proto: string;
    message: string;
  }
};

type Return = {
  type: string;
  proto: string;
  message: string;
};

type CacheModule = {
  index: number;
  module: any;
  name: string;
};

type Request = {
  jsonrpc: string;
  module: string;
  name: string;
  index: number;
  args: any[];
  type: string;
};

type Response = {
  jsonrpc: string;
  index: number;
  code: number;
  result: any;
};
