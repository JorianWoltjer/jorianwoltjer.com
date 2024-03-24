import { useEffect, useState } from "react";
import { timeDifference } from "@/utils/strings";

export default function RelativeTime({ timestamp, interval_ms = 1000 }) {
  const [now, setNow] = useState(Date.now())

  useEffect(() => {
    if (timestamp === undefined) return;
    setInterval(() => setNow(Date.now()), interval_ms)
  }, [timestamp, interval_ms])

  if (timestamp === undefined) return "0 seconds ago"

  return <span suppressHydrationWarning>
    {timeDifference(new Date(timestamp), now)}
  </span>
}
