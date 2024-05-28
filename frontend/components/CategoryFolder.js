import { faFlag, faLaptopCode, faTerminal } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Link from "next/link";

const icons = {
  "flag": faFlag,
  "terminal": faTerminal,
  "laptop": faLaptopCode
}

export default function CategoryFolder({ slug, img, title }) {
  const href = slug ? `/blog/f/${slug}` : "#";

  return <Link className="big-button" href={href}>
    <FontAwesomeIcon icon={icons[img]} />
    {title}
  </Link>
}