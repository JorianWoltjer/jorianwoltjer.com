import { Metadata } from "@/components";
import { BACKEND_API } from "@/config";
import { useRouter } from 'next/router';
import { useState } from "react";

export default function Login() {
  const router = useRouter();
  const [alert, setAlert] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setAlert(null);

    const { password } = e.target;
    fetch(BACKEND_API + "/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",  // Receive and save cookies
      body: JSON.stringify({
        password: password.value
      })
    }).then(res => {
      if (res.ok) {
        document.cookie = "admin_interface=true; Path=/; SameSite=Lax; Secure";
        document.location.href = /^\/[a-z]/.test(router.query.next) ? router.query.next : "/blog";
      } else {
        setAlert(<div className="alert alert-danger" role="alert">Incorrect password</div>);
      }
    });
  }

  return <>
    <Metadata title="Login" description="Log into the administrator interface used to manage this website" />
    <div className="boxed center-transform">
      <h1>Admin login</h1>
      <br />
      {alert}
      <form method="post" onSubmit={handleSubmit}>
        <input className="form-control" type="password" id="password" name="password" placeholder="Password..." autoFocus />
        <br />
        <input className="btn btn-secondary" type="submit" value="Submit" />
      </form>
    </div>
  </>
}
