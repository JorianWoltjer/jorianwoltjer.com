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
