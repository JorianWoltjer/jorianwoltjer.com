import { ErrorPage } from "@/components";
import { useEffect, useState } from "react";

export default function NotFound() {
    const [path, setPath] = useState("");

    useEffect(() => {
        setPath(window.location.pathname);
    }, []);

    return <ErrorPage
        title="Not Found"
        message={<>The path <code suppressHydrationWarning>{path}</code> does not exist. </>}
    />
}
