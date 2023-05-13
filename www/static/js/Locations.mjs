import 'https://maps.googleapis.com/maps/api/js?key=AIzaSyASGYgBONrHLT6c4ZvqH5vKn1XMe3ChKsI';

class Locations {
    constructor() {}

    /**
     * @method loadMap
     * @param {Object} arg
     * @param {number} arg.lat
     * @param {number} arg.lng
     * @param {HTMLElement} arg.cont
     * @param {string} arg.title
     * @returns void
     * */
    async loadMap({lat,lng,cont,title}){
        const latLng = await new google.maps.LatLng(lat, lng);
        const config = {
            zoom: 12,
            center: latLng,
            mapTypeId: google.maps.MapTypeId.ROADMAP
        };
        const map = await new google.maps.Map(cont, config);

        const marker = await new google.maps.Marker({position: latLng, map, title});

        google.maps.event.addListener(marker, 'click', function() {
            const infoWindow = new google.maps.InfoWindow({content: title,});
            infoWindow.open(map,marker);
        });
    }

    /**
     * @init
     * @async
     * */
    async init(){
        this.loadMap({
            cont    : document.getElementById('map_canvas_1'),
            title   : "Headquarters\n(Corporate Office)",
            lat     : 42.561836,
            lng     : -83.434562,
        });

        this.loadMap({
            cont    : document.getElementById('map_canvas_2'),
            title   : "Wholesale/Distribution",
            lat     : 43.730420,
            lng     : -79.558550,
        });

        this.loadMap({
            cont    : document.getElementById('map_canvas_3'),
            title   : "Wholesale/Distribution",
            lat     : 39.996749,
            lng     : -75.0953099,
        });
    }
}
export default Locations;