export function slugify(title) {
    const regex = /(<.*?>)|(&.*?;)|[^\w]+/g;
    const slug = title
        .replace(regex, '-')
        .replace(/^-+|-+$/g, '')
        .toLowerCase();
    return slug;
}

export function xmlEscape(obj) {
    if (typeof obj === 'string') {
        return obj.replace(/&/g, '&amp;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&apos;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
    } else if (typeof obj === 'object') {
        for (const key in obj) {
            obj[key] = xmlEscape(obj[key])
        }
    }
    return obj
}

export function timeDifference(timestamp, from) {
    from = from || Date.now();
    from = new Date(from).getTime();
    timestamp = new Date(timestamp).getTime();

    const msDiff = Math.abs(from - timestamp);

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

    value = Math.floor(value); // Round down
    unit += value === 1 ? "" : "s";

    if (timestamp > from) {
        return "in " + value + " " + unit;
    } else {
        return value + " " + unit + " ago";
    }
}

export function toLocalTime(datetime) {
    const timezoneOffset = new Date().getTimezoneOffset() * 60000;
    return new Date(datetime - timezoneOffset).toISOString().slice(0, -8)
}
