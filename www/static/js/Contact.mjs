const reCAPTCHA_site_key = "6Lc_rfolAAAAAODnXGRk4Ev32b61gkx8Vihc197t";
class Contact {
    constructor() {
        /** @type {HTMLFormElement || null}*/ this.formElem = null;
    }

    /**
     * @method sendEmail
     * @param {Object} arg
     * @param {string} [arg.url=""]
     * @async
     * */
    async sendEmail({url="",data={}}){
        const urlEncodedData = new URLSearchParams([...data.entries()]).toString();
        const response = await fetch(url, {
            method: "POST",
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: urlEncodedData,
        });
        return response.json();
    }


    /**
     * @module bindFormSubmit
     * @description extends on form submit. Uses reCAPTCHA v3 to validate
     * */
    bindFormSubmit(){
        const formElem = document.getElementById('contact_form');

        formElem.onsubmit =(event)=>{
            event.preventDefault();

            grecaptcha.enterprise.ready(()=>{
                grecaptcha.enterprise.execute(reCAPTCHA_site_key, { action: 'submit' })
                    .then((token)=> {
                        document.getElementById('g_recaptcha_response').value = token;
                        const formData = new FormData(formElem);
                        const data = {};
                        for (const [key,val] of formData.entries()) data[key] = val;

                        this.sendEmail({url:"/submit_form",data:formData})
                            .then(({message, status})=>{
                                window.alert(message);
                                if(status === 'success') this.formElem.reset();
                            });
                    });
            });
        };
        return formElem;
    }

    /** @module init */
    init(){
        const recaptchaScript = document.createElement('script');
        recaptchaScript.src = `https://www.google.com/recaptcha/enterprise.js?render=${reCAPTCHA_site_key}`;
        document.head.appendChild(recaptchaScript);

        this.formElem = this.bindFormSubmit();
    }
}
export default Contact;