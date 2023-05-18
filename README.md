# mopartsusa

[mopartsusa.com](https://www.mopartsusa.com/)\
[WayBackMachine](https://web.archive.org/web/20230000000000*/mopartsusa.com)

MopartsUSA, born in 2013, was initially an experimental project exploring minimalistic web design principles. 
It was constructed with HTML5 markup language and CSS3 for styling—innovative but not yet standard technologies at the time. 
Now, a decade later, this project is due for a refresh, transitioning from an in-house server to a robust Google Cloud Platform (GCP) instance.

The primary objective of this revision is to demonstrate the basic requirements needed to create a fully static, mobile-responsive, user-friendly website, all without leaning on any JavaScript-specific frameworks. 
This guide is designed as a resource for aspiring developers keen to explore the Rust language, providing an illustrative example rather than a comprehensive manual.

Here's what I'll be using:

1. [Rust](https://www.rust-lang.org/) - Rust, known for its impressive performance and safety features, especially regarding safe concurrency, will be our choice for backend templating.
2. [Less](https://lesscss.org/) -  I will use Less, a dynamic preprocessor style sheet language, to extend the capabilities of CSS, facilitating more adaptable and maintainable stylesheets.
3. [vanilla-js](http://vanilla-js.com/) - To avoid heavy, vendor-specific JavaScript frameworks, I am turning to vanilla-js, lightweight and fast.
4. [Google Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/overview) - This API will empower us to personalize maps with our content and imagery, enabling the integration of interactive maps on our web pages.
5. [reCAPTCHA Enterprise](https://cloud.google.com/recaptcha-enterprise) - I'll be using reCAPTCHA Enterprise to protect our website's email system from spam.
6. [NGINX](https://www.nginx.com/) - Serving as the reverse proxy, NGINX will manage our HTTP traffic, providing security, scalability, and routing functionality. Also using NGINX for serving static files.
7. [HTTP2](https://en.wikipedia.org/wiki/HTTP/2) I'll include the HTTP/2 protocol for its benefits, such as multiplexing, server push, and header compression.
8. [Certbot](https://certbot.eff.org/) Certbot will be used to obtain HTTPS certificates, securing encrypted communications.

## Google Cloud (Compute Engine)
- **Zone**: us-central1-a
- **Machine type**: e2-micro 0.25-2 vCPU 1 shared core, 1GB memory ~$8.11 per month
- **Server**: CentOS Stream 9 

While this tech stack might seem overkill for a small project like MopartsUSA, remember that the purpose is educational. 
This project is an opportunity to dive into the use of Rust and other technologies in a real-world scenario. 
You will have a better understanding of how these pieces fit together to build a responsive, user-friendly and fast website.