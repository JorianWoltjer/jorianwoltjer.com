UPDATE folders SET title = 'CTF', slug = 'ctf', img = 'fa-solid fa-flag', description = 'Writeups from Capture the Flag (CTF) events I participated in. About web, scripting, crypto, reversing and everything in between. All writeups include detailed explanations of how everything works.'
  WHERE id = 1;
UPDATE folders SET title = 'Research', slug = 'research', img = 'fa-solid fa-flask', description = 'Pieces of research containing novel or otherwise interesting techniques. Often contains real-world examples but is focused on the new technique that isn''t widely known yet.'
  WHERE id = 2;
UPDATE folders SET title = 'Stories', slug = 'stories', img = 'fa-solid fa-comment-dots', description = 'Stories from real-world hacking, coding, and other projects I''ve done. This shows what can practically be found in the wild, as well as some meta posting about this site.'
  WHERE id = 3;