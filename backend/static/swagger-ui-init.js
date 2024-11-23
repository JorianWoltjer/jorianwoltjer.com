window.onload = () => {
  window.ui = SwaggerUIBundle({
    url: "swagger.json",
    basePath: "/api",
    dom_id: "#swagger-ui",
  });
};