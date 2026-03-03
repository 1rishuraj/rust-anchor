import * as borsh from "borsh";

export class CounterAccount {
  count: number;
  constructor(fields: { count: number } = { count: 0 }) {
    this.count = fields.count;
  }
}

export const schema = new Map([
  [CounterAccount, { kind: "struct", fields: [["count", "u32"]] }],
]);

export const Counter_Size = borsh.serialize(schema, new CounterAccount()).length;
