import { BACKEND_API } from "@/config";
import { useRouter } from 'next/router'

export default function Login() {
    const router = useRouter();

    const handleSubmit = async (e) => {
        e.preventDefault();

        const { password } = e.target;
        fetch(BACKEND_API + "/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            credentials: "same-origin",  // Receive and save cookies
            body: JSON.stringify({
                password: password.value
            })
        }).then(res => {
            if (res.ok) {
                const expires = new Date();
                expires.setDate(expires.getDate() + 1);  // Should align with sid= cookie expiration
                document.cookie = "admin_interface=true; Path=/; Expires=" + expires.toUTCString();
                document.location.href = router.query.next || "/blog";
            }
        });
    }

    return <>
        <h1>Login</h1>

        <form onSubmit={handleSubmit}>
            <input name="password" type="password" placeholder="Password" /><br />
            <button type="submit">Submit</button>
        </form>
    </>
}
