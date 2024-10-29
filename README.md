# BlogProject
Blog Engine is a Rust-based web application that aggregates and displays articles across various categories, including Technology, Entertainment, and Articles. This project leverages the Actix-web framework to serve dynamic content fetched from external APIs, specifically NewsAPI.

## Features
**Technology News:** Fetches and displays the latest news in technology. <br>
**Entertainment News:** Provides up-to-date articles from the entertainment industry. <br>
**Articles:** Provides random articles to read from around the world. <br>
**Home Page Navigation:** Easy links to navigate between different sections. <br>
**Responsive Design:** Monochrome clours with design that can be viewed on any device efficiently. <br>

## Requirements
**Rust:** Ensure you have Rust installed. `Install Rust`<br>
**Cargo:** Rust's package manager, included with Rust.
### Installation
Clone the Repository:
`git clone https://github.com/yourusername/BlogProject.git` <br>
`cd BlogProject`

### 2 Install Dependencies:
`cargo build`
**Configuration**
**API Key:** Sign up at NewsAPI to get an API key. Replace the placeholder API key in `src/routes/articles.rs`, `src/routes/technologies.rs`, and any other relevant files with your personal API key.
## Usage
### Run the Server:
`cargo run`
### Access the Application:
- Open your web browser.
- Navigate to `http://localhost:8080` to view the homepage.
- From the homepage, click on links to navigate to Articles, Technology News, and Entertainment News sections.
### Project Structure
- `src/main.rs`: Initializes the server and configures routes.
- `src/routes/mod.rs`: Manages routing for different content sections.
- `src/routes/articles.rs`: Handles fetching and displaying articles.
- `src/routes/technologies.rs`: Manages technology news content.
- `src/routes/entertainment.rs`: Displays entertainment news.
- `static/styles.css`: Contains styles for the user interface.
### Contributing
Contributions are highly welcome! Feel free to fork the repository and create a pull request with feature additions, bug fixes, or improvements.
## License
This project is licensed under the MIT License. See the LICENSE file for details.
## Acknowledgments
**Rust Programming Language:** The systems programming language empowering this project. <br>
**Actix-web Framework:** The web framework for handling HTTP requests in Rust. <br>
**NewsAPI:** Providing access to a wide range of news articles. <br>
