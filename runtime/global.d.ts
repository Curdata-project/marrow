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
};

type Return = {
  type: string;
  proto: string;
  message: string;
};

type CacheModule = {
  index: number;
  targetMethod: Expose;
}
