import Image from "next/image";

function capitalize(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

export default function Project({ title, text, img, href, category }) {
    return <div className="col-lg-4 col-sm-6 mb-4">
        <div className="card h-100">
            <a href={href} target={(!href || category == "Utility") ? "" : "_blank"}>
                <div className="card-img-top">
                    <Image fill src={`http://nginx/img/projects/${img}`} alt="Project thumbnail" />
                </div>
            </a>
            <div className="card-body">
                <p className="card-text tags">
                    <span className={`tag tag-${category}`}>{capitalize(category)}</span>
                </p>
                <h4 className="card-title">
                    <a href={href} target={(!href || category == "Utility") ? "" : "_blank"}>{title}</a>
                </h4>
                <p className="card-text" dangerouslySetInnerHTML={{ __html: text }}></p>
            </div>
        </div>
    </div>
}
