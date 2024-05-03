import { Vent } from "./vent";

describe("test Vent class", () => {
  it("should create a new Vent", () => {
    const input = "8,0 -> 0,8";
    const vent = new Vent(input);

    expect(vent.start[0]).toStrictEqual(8);
    expect(vent.start[1]).toStrictEqual(0);
    expect(vent.end[0]).toStrictEqual(0);
    expect(vent.end[1]).toStrictEqual(8);
  });

  it("shouldn't create a new Vent", () => {
    const inputs = ["8, -> 0,8", "8,0 -> ,8", "hello"];

    for (const input of inputs) {
      expect(() => new Vent(input)).toThrow();
    }
  });
});
