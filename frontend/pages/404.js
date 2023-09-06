import { ErrorPage } from "@/components";
import { useRouter } from "next/router";

export default function NotFound() {
    const router = useRouter();

    return <ErrorPage
        title="Not Found"
        message={<>The path <code suppressHydrationWarning>{router.asPath}</code> does not exist. </>}
    />
}
