export default class Formatter {
  // format number to compact
  public static formatNumber(num: number): string {
    const formatter: Intl.NumberFormat = Intl.NumberFormat("en-us", {
      notation: "compact",
    });
    return formatter.format(num);
  }

  // format currency
  public static formatCurrency(amount: number, isCompact = false): string {
    const options: any = {
      style: "currency",
      currency: "USD",
    };

    if (isCompact) {
      options.notation = "compact";
    }

    const formatter: Intl.NumberFormat = Intl.NumberFormat("en-us");
    return formatter.format(amount);
  }
}
