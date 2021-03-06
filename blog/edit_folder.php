<?php
if (!isset($create_folder)) $create_folder = false;

$admin_required = true;
$meta_title = ($create_folder ? "Create" : "Edit")." folder";
$meta_description = "Form to ".($create_folder ? "create" : "edit")." a folder for posts on my blog.";
require_once("../include/all.php");

if (!$create_folder) {
    $response = sql_query("SELECT * FROM folders WHERE id = ?", [$_GET['id']]);
    $row = $response->fetch_assoc();

    if ($response->num_rows === 0) {
        returnMessage("error_folder", "/blog/");
    }
} else {
    $row = [
        "title" => "",
        "description" => "",
        "img" => "../placeholder.png",
        "parent" => $_GET["parent"] ?? null,
    ];
}

if ($_SERVER["REQUEST_METHOD"] === "POST") {
    if (isset($_POST['title'], $_POST['description'], $_POST['image'], $_POST['parent'])) {
        $url = text_to_url($_POST["title"]);
        if ($_POST['parent'] !== "") {
            $parent = sql_query("SELECT url FROM folders WHERE id=?", [$_POST["parent"]]);
            $parent_url = $parent->fetch_assoc()["url"];
            $url = $parent_url."/".$url;
        } else {
            $_POST['parent'] = null;
        }

        if ($create_folder) {
            sql_query("INSERT INTO folders(title, description, img, url, parent, timestamp) VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP())",
                [$_POST['title'], $_POST['description'], $_POST['image'], $url, $_POST['parent']]);
        } else {
            $response = sql_query("UPDATE folders SET title=?, description=?, img=?, url=?, parent=? WHERE id=?",
                [$_POST["title"], $_POST["description"], $_POST["image"], $url, $_POST["parent"], $_GET["id"]]);
        }

        header("Location: /blog/folder/".$url);
        exit();
    }
}

require_once("../include/header.php");
?>

    <h1 class="my-4"><code><?= $create_folder ? "Create" : "Edit" ?> folder</code></h1>

    <form method="POST" id="form">
        <label for="title">Title</label>
        <input class="form-control" id="title" type="text" name="title" required autocomplete="off" autofocus value="<?= $row["title"] ?>">
        <br>
        <label for="description">Description</label>
        <textarea class="form-control" id="description" name="description" spellcheck="true" rows="2" required><?= $row["description"] ?></textarea>
        <br>
        <label for="image">Image</label>
        <input class="form-control" id="image" type="text" name="image" autocomplete="off" value="<?= $row["img"] ?>">
        <br>
        <img id="preview" src="/img/blog/<?= $row["img"] ?>" alt="Unable to load image!" class="rounded" width="300px">
        <br>
        <br>
        <label for="parent">Parent folder</label>
        <select class="form-control" id="parent" name="parent">
            <option value=""></option>
            <?php
            $response = sql_query("SELECT id, title FROM folders");

            while($row_folder = $response->fetch_assoc()) {
                if ($row_folder['id'] == $row['parent']) {
                    echo "<option value='$row_folder[id]' selected>$row_folder[title]</option>";
                } elseif ($row_folder['id'] !== $row['id']) {
                    echo "<option value='$row_folder[id]'>$row_folder[title]</option>";
                }
            }
            ?>
        </select>
        <br>
        <input class="btn btn-primary" type="submit" name="submit" value="Save">
    </form>

    <script nonce="<?=$nonce?>">
        $('#image').on("change", function() {
            const src = $(this).val();
            $("#preview").attr('src', "/img/blog/"+src);
        });
    </script>

<?php require_once("../include/footer.php"); ?>