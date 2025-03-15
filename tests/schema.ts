export class Counter {
    count = 0;
    constructor(fields: { count: number } | undefined = undefined) {
      if (fields) {
        this.count = fields.count;
      }
    }
  }
  
export const CounterSchema = new Map([
  [Counter, { kind: "struct", fields: [["count", "u64"]] }],
]);

export enum CounterInstruction {
    Initialize = 0,
    IncreaseCounter = 1,
    Delegate = 2,
    CommitAndUndelegate = 3,
    Commit = 4,
    Undelegate = 5,

}

export class IncreaseCounterPayload {
    increase_by: number;

    constructor(increase_by: number) {
        this.increase_by = increase_by;
    }

    static schema = new Map([
        [
        IncreaseCounterPayload,
        {
            kind: 'struct',
            fields: [
            ['increase_by', 'u64'],
            ],
        },
        ],
    ]);
}