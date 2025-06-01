UPDATE folders SET title = 'CTF', slug = 'ctf', img = 'fa-solid fa-flag', description = 'Writeups from Capture the Flag (CTF) events I participated in. About web, scripting, crypto, reversing and everything in between. All writeups include detailed explanations of how everything works.'
  WHERE id = 1;
UPDATE folders SET title = 'Research', slug = 'research', img = 'fa-solid fa-flask', description = 'Generic techniques that are not yet very well known. Contains novel or interesting pieces of research by me to help advance the field.'
  WHERE id = 2;
UPDATE folders SET title = 'Stories', slug = 'stories', img = 'fa-solid fa-comment-dots', description = 'Stories from real-world hacking, coding, and other projects I''ve done. These are often more realistic but less crazy.'
  WHERE id = 3;