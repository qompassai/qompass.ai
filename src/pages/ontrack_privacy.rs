use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn OnTrackPrivacy() -> impl IntoView {
    view! {
        <Title text="ONTrack Privacy - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — ONTRACK PRIVACY"</p>
            <h1>"ONTrack Privacy Policy"</h1>
            <div class="prose">
                <p>"Last updated: October 13, 2026."</p>
                <p>
                    "This Privacy Policy explains how ONTrack (“the App”) handles "
                    "information when you use it. By using the App, you agree to the "
                    "practices described in this Policy."
                </p>
                <p>"If you do not agree with this Policy, please do not use the App."</p>
                <h2>"1. Information We Collect"</h2>
                <h3>"1.1 Information You Provide"</h3>
                <p>"The App collects information that you voluntarily enter, such as:"</p>
                <p>"Job addresses, stop lists, and route configuration you add to plan a route."</p>
                <p>
                    "Job and address lists you import from a file you select (CSV or "
                    "spreadsheet). The App reads the file on your device to build your "
                    "stop list — the file itself is never uploaded anywhere — and the "
                    "addresses it contains are handled exactly like addresses you type, "
                    "including Section 3 when they are geocoded or routed. On Android, "
                    "reading a file outside the App's own storage uses the storage "
                    "permission on API 32 and below."
                </p>
                <p>"Feedback you send us by email or other support channels."</p>
                <p>
                    "All of this data is stored locally on your device inside the App's "
                    "own storage. ONTrack has no account system and no backend server of "
                    "its own — there is nothing to sync it to."
                </p>
                <h3>"1.2 Automatically Collected Information"</h3>
                <p>
                    "The App does not use analytics or crash-reporting SDKs. If a future "
                    "release adds any, this Policy will be updated first and the change "
                    "will be reflected in the Play Store Data Safety section before it ships."
                </p>
                <h3>"1.3 Location Data"</h3>
                <p>
                    "If you grant the Location permission, the App reads your device's "
                    "last known GPS/network location (via Android's LocationManager) to "
                    "center the map on your current position and to optionally use it as "
                    "a route start point. This location is used on-device only and is not "
                    "transmitted to us — it is never sent to any ONTrack server, because "
                    "none exists. It may be sent to a routing/geocoding provider as "
                    "described in Section 3 below if you use those features with your "
                    "current location as a stop."
                </p>
                <h3>"1.4 Voice Input"</h3>
                <p>
                    "If you grant the Microphone permission, the App can transcribe "
                    "spoken stop entries to text using an on-device Whisper (whisper.cpp) "
                    "model that you download once during setup. Audio is processed "
                    "entirely on your device and is never uploaded anywhere."
                </p>
                <h2>"2. How We Use Information"</h2>
                <p>"We use the information described above to:"</p>
                <p>"Provide and operate the App's route-planning and optimization features."</p>
                <p>
                    "Geocode the addresses you enter into map coordinates, and compute "
                    "drive-time/distance between your stops, using the third-party "
                    "services listed in Section 3."
                </p>
                <p>"Respond to user requests, questions, or support inquiries."</p>
                <p>
                    "We do not sell your personal data, and we have no analytics pipeline "
                    "of our own to sell it from."
                </p>
                <h2>"3. Third-Party Services"</h2>
                <p>
                    "ONTrack has no backend server. To turn the addresses you type into "
                    "map coordinates and routes, the App sends the address text or "
                    "coordinates you enter directly from your device to one of the "
                    "following, depending on which backend you select in Settings:"
                </p>
                <p>
                    <strong>"OpenStreetMap Nominatim (nominatim.openstreetmap.org). "</strong>
                    "— default geocoding backend. Receives the address text you enter. "
                    "See the "
                    <a href="https://osmfoundation.org/wiki/Privacy_Policy">
                        "OSMF Privacy Policy"
                    </a>
                    "."
                </p>
                <p>
                    <strong>"Project OSRM public demo server (router.project-osrm.org). "</strong>
                    "— default routing backend. Receives the coordinates of your stops "
                    "to compute a distance/time matrix."
                </p>
                <p>
                    <strong>"ip-api.com. "</strong>
                    "— used only as a coarse fallback to estimate your approximate "
                    "location from your network connection when device GPS is unavailable. "
                    "Receives your device's public IP address."
                </p>
                <p>
                    <strong>"Google Maps Platform (Geocoding API and Distance Matrix API). "</strong>
                    "— optional, only used if you supply your own Google Maps API key in "
                    "Settings and select Google as the backend. Receives the address text "
                    "and/or coordinates of your stops. See "
                    <a href="https://policies.google.com/privacy">"Google's Privacy Policy"</a>
                    "."
                </p>
                <p>
                    <strong>"Google Play. "</strong>
                    "— used by Android itself for App distribution and updates, "
                    "independent of the App's own code."
                </p>
                <p>
                    "We do not control these third parties' own data retention or use of "
                    "the data once it is sent to them; please refer to their privacy "
                    "policies linked above. You can avoid all of them by entering "
                    "coordinates directly instead of addresses, though drive-time "
                    "optimization requires at least one routing backend."
                </p>
                <h2>"4. Data Storage and Retention"</h2>
                <p>
                    "Job addresses, stop lists, and settings are stored locally on your "
                    "device only, inside the App's private storage, until you delete them "
                    "or uninstall the App. We do not operate any server that stores this "
                    "data. We retain nothing on our end beyond email correspondence you "
                    "initiate with us."
                </p>
                <h2>"5. Data Sharing and Disclosure"</h2>
                <p>"We may share information in the following limited situations:"</p>
                <p>
                    <strong>"Geocoding/routing providers: "</strong>
                    "With the OpenStreetMap Nominatim, OSRM, ip-api.com, or Google Maps "
                    "services described in Section 3, only the address text or coordinates "
                    "needed to fulfill your request."
                </p>
                <p>
                    <strong>"Legal requirements: "</strong>
                    "When required to comply with applicable law, legal processes, or "
                    "valid governmental requests."
                </p>
                <p>
                    <strong>"Protection of rights: "</strong>
                    "When necessary to protect our rights, property, safety, or that of "
                    "users or the public."
                </p>
                <p>
                    "We do not share your personal data with third parties for their own "
                    "marketing purposes, and we do not sell it."
                </p>
                <h2>"6. Your Choices and Controls"</h2>
                <p>"You can:"</p>
                <p>
                    "Control the Location and Microphone permissions via your device's "
                    "App settings at any time; the App degrades gracefully without them "
                    "(you can type addresses and stop entries manually)."
                </p>
                <p>
                    "Choose which geocoding/routing backend to use (or enter coordinates "
                    "directly to avoid sending address text to any third party) in the "
                    "App's Settings."
                </p>
                <p>
                    "Delete local app data by clearing the app's storage or uninstalling "
                    "the App from your device — this removes all locally stored addresses, "
                    "routes, and settings."
                </p>
                <p>
                    "If you contact us directly, you may request deletion or clarification "
                    "of information you have provided, where technically feasible and not "
                    "otherwise required to be retained by law."
                </p>
                <h2>"7. Children's Privacy"</h2>
                <p>
                    "The App is not intended for children under 13 (or a higher age where "
                    "required by local law), and we do not knowingly collect personal data "
                    "from children in that age group. If we learn that we have inadvertently "
                    "collected data from a child, we will take reasonable steps to delete it."
                </p>
                <h2>"8. Security"</h2>
                <p>
                    "We take reasonable technical and organizational measures to protect "
                    "information processed by the App against unauthorized access, loss, or "
                    "misuse. However, no method of transmission or storage is completely "
                    "secure, and we cannot guarantee absolute security."
                </p>
                <h2>"9. Changes to This Policy"</h2>
                <p>
                    "We may update this Privacy Policy from time to time. When we do, we "
                    "will revise the “Last updated” date at the top of this page. Your "
                    "continued use of the App after any changes means you accept the "
                    "updated Policy."
                </p>
                <h2>"10. Contact Us"</h2>
                <p>
                    "If you have questions about this Privacy Policy or about how ONTrack "
                    "handles data, you can contact us at:"
                </p>
                <p>
                    "Email: "
                    <a href="mailto:matt@aflabs.io">"matt@aflabs.io"</a>
                    "."
                </p>
                <p>
                    "Please include “ONTrack Privacy” in the subject line so we can "
                    "process your request more efficiently."
                </p>
            </div>
        </section>
    }
}
