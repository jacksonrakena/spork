# Project Spork
Project Spork is an open government project aimed at scraping data about the New Zealand Parliament, specifically the House of Representatives (the only voting chamber).
  
At the moment, the project goals are to:
- [x] Scrape a list of all active MPs
- [ ] Allow users to see how members voted (in conscience votes), or how their party voted (in regular votes)
  - [ ] Correctly list and highlight defections, floor crossings, and abstentions on the floor without proper reason
- [ ] Automatically scrape Hansard debate content and match the speaker content to an MP in the database
  - [ ] Allow users to highlight and comment on specific text, as well as share an easy link to a specific
    phrase said by a member
- [ ] Provide this data in an open-source format, as well as an API for other services to access
  
## Architecture
**scraping_tools**  
A Rust-based set of command line tools to scrape [Hansard Debate Reports](https://www.parliament.nz/en/pb/hansard-debates/rhr/), the [Data Service](https://catalogue.data.govt.nz/dataset/members-of-parliament/resource/89069a40-abcf-4190-9665-3513ff004dd8),
and other Parliament data sources for information.

**Server**  
A likely Rust (`actix`) based site that serves content scraped by the scraping tools, and connects to the database.  
Backed by a Postgres database server, which will be regularly mirrored to this GitHub repository for offline analysis.  
  
**Client**  
A React SPA that serves data from the API in an easy-to-use, user-friendly format, that aims to be a modern implementation of [They Work For You](https://www.theyworkforyou.com/), a similar website for the Parliament of the United Kingdom.

## Inspiration
This project is heavily inspired by [They Work For You](https://www.theyworkforyou.com/), a highly similar service tracking debates in the United Kingdom.

## Copyright
All scraped data, exported content, and database information is copyright of their respective owners, which is usually (but not always) the public domain in New Zealand.  
Project Spork's code and assets are licensed under the MIT License.
