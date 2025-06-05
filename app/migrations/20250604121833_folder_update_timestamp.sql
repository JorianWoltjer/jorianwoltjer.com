CREATE OR REPLACE FUNCTION update_folder_timestamp()
RETURNS trigger AS $$
BEGIN
  -- Update folder timestamp to the new post timestamp
  UPDATE folders
  SET timestamp = NEW.timestamp
  WHERE id = NEW.folder AND NOT NEW.hidden;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER post_timestamp_update_trigger
AFTER INSERT OR UPDATE OF timestamp ON posts
FOR EACH ROW
EXECUTE FUNCTION update_folder_timestamp();
