import { useEffect, useState } from "react";

export function timeDifference(current, timestamp) {
  const msDiff = current - timestamp;

  const msPerMinute = 60 * 1000;
  const msPerHour = msPerMinute * 60;
  const msPerDay = msPerHour * 24;
  const msPerMonth = msPerDay * 30;
  const msPerYear = msPerDay * 365;

  let value, unit;
  if (msDiff < msPerMinute) {
    value = msDiff / 1000;
    unit = "second";
  } else if (msDiff < msPerHour) {
    value = msDiff / msPerMinute;
    unit = "minute";
  } else if (msDiff < msPerDay) {
    value = msDiff / msPerHour;
    unit = "hour";
  } else if (msDiff < msPerMonth) {
    value = msDiff / msPerDay;
    unit = "day";
  } else if (msDiff < msPerYear) {
    value = msDiff / msPerMonth;
    unit = "month";
  } else {
    value = msDiff / msPerYear;
    unit = "year";
  }

  unit += Math.round(value) === 1 ? "" : "s";

  return Math.round(value) + " " + unit + " ago";
}

export default function RelativeTime({ timestamp, interval_ms = 1000 }) {
  const [now, setNow] = useState(Date.now())

  useEffect(() => {
    if (timestamp === undefined) return;
    setTimeout(() => setNow(Date.now()), interval_ms)
  }, [timestamp, interval_ms])

  if (timestamp === undefined) return "0 seconds ago"

  return <span suppressHydrationWarning>
    {timeDifference(now, new Date(timestamp))}
  </span>
}
