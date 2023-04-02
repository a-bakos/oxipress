# WIP Rust CMS
## A thought experiment project based on WordPress

This is a project for myself to practice and grow my Rust skills. The main idea behind this project is to explore and
re-create some parts of WordPress using Rust in order to pick up new things, solidify what I've learnt, understand and commit to muscle memory.

**Aims and limitations:**

Eventually, I want a prototype for a basic CMS flow. _However..._

Certainly, re-implementing a WordPress-like CMS is an enormous undertaking, something I wouldn't be able to do alone within a sensible timeframe.

However, at this point, it's not my goal to do that. I'm also not saying that I wouldn't want it to be my goal in the future. 

Right now, my aim is to learn about Rust, practice it regularly, and in varied ways - and for that purpose an extensive CMS (which nowadays is more like a framework) is a great subject.

Cherrypicking certain parts for re-implementation is a great method for me to keep the project and learning interesting. It is great exercise to think critically about data structures and modelling. 

Even a small module, like permalink-generation has a lot of hidden knowledge, something that has already taught me a great deal and not just about Rust per se. 

I'm not concerned with an asynchronous approaches right now. I do want to look at that soon, though. 

Database-wise, I'm starting with one of the easiest possible ways, using a simple CSV format to store post data. Later, I will look into real solutions, like PostgreSQL or SurrealDB. 

**What if...**

Who knows/wishful thinking, etc, and it's a big _if_ at this point, but if I can reach escape velocity with this prototyping exercise, there's always the potential it might even grow into a real product...
