interface Interval {
  label: String;
  seconds: number;
}
const intervals: Interval[] = [
  { label: "year", seconds: 31536000 },
  { label: "month", seconds: 2592000 },
  { label: "day", seconds: 86400 },
  { label: "hour", seconds: 3600 },
  { label: "minute", seconds: 60 },
  { label: "second", seconds: 1 }
];

export function timeSince(timestamp?: number): string {
  if (timestamp === undefined) {
    return "";
  } else {
    const date = new Date(timestamp * 1000);
    const seconds = Math.floor((Date.now() - date.getTime()) / 1000);
    const interval: Interval | undefined = intervals.find(
      (i) => i.seconds < seconds
    );
    if (interval) {
      const count = Math.floor(seconds / interval.seconds);
      return `${count} ${interval.label}${count !== 1 ? "s" : ""} ago`;
    }
    return "Invalid date";
  }
}

export const getCurrentYear = () => new Date().getFullYear();

export const getMonthsSinceLaunch = () => {
  // Site launched in Dec-2022
  const FROM_YEAR = 2022;
  const FROM_MONTH = 12;
  const currentDate = new Date();
  const toYear = currentDate.getFullYear();
  const toMonth = currentDate.getMonth();
  // Include start month
  const months = [{ year: FROM_YEAR, month: FROM_MONTH }];

  // Add the months since the start month to current month
  for (let year = FROM_YEAR; year <= toYear; year++) {
    let monthNum = year === FROM_YEAR ? FROM_MONTH : 0;
    const monthLimit = year === toYear ? toMonth : 11;

    for (; monthNum <= monthLimit; monthNum++) {
      let month = monthNum + 1;
      months.push({ year, month });
    }
  }
  return months;
};
