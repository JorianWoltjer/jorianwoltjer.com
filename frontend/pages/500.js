import { ErrorPage } from "@/components";

export default function InternalServerError() {
    return <ErrorPage
        title="Internal Server Error"
        message={<>There was an error loading this page.</>}
    />
}
