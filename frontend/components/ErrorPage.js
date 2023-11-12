import { faTriangleExclamation } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Link from "next/link";
import { useRouter } from "next/router";

export default function ErrorPage({ title, message }) {
    const router = useRouter();

    return <div className="center-transform">
        <FontAwesomeIcon icon={faTriangleExclamation} className="big-icon" />
        <h1>{title}</h1>
        <p className="lead">
            {message}
            <br />
            You can try going back <Link href="/">Home</Link> or to the <a href="#" onClick={router.back}>previous page</a>.
        </p>
        <p className="lead text-muted">
            (or create an <Link style={{ color: 'inherit' }} href="https://github.com/JorianWoltjer/jorianwoltjer.com/issues/new">Issue</Link> with context if you think this is a bug)
        </p>
    </div>
}
