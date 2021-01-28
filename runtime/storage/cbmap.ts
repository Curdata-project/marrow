const cbmap = new Map();

type MapPayload = {
  cb: number;
  user_data: number;
  moduleName: string;
  funcName: string;
};

export const getIndex = (index: number) => {
  return cbmap.get(index);
};

export const setIndex = (index: number, payload: MapPayload) => {
  cbmap.set(index, payload);
};
