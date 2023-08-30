export default function timeAgo(timestamp) {
  const currentDate = new Date();
  const targetDate = new Date(timestamp);
  const msDiff = currentDate - targetDate;

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
