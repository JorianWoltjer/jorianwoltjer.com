import '@/styles/globals.css'
import { BACKEND_API } from "@/config";
import { useEffect, useState } from "react";

export default function App({ Component, pageProps }) {
  const [admin_interface, set_admin_interface] = useState(false);

  useEffect(() => {
    if (document.cookie.includes("admin_interface=true")) {
      set_admin_interface(true);
    }
  }, [])

  const logout = async () => {
    await fetch(BACKEND_API + "/logout", {
      method: "GET",
      credentials: "same-origin"
    })
    document.cookie = "admin_interface=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";

    set_admin_interface(false);
  }

  return <>
    {admin_interface && (<>
      <a onClick={logout} href="#">Logout</a>
    </>)}
    <Component {...pageProps} admin_interface={admin_interface} />
  </>
}
