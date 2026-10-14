//! The identity section of /about (anchor `#identity`).
//!
//! The mailbox, the `OpenPGP` key, and every identifier in content/cv.toml
//! twice over — once as ticks on a slowly drifting compass card, once as
//! a plain prerendered list. The list is the guarantee: complete in the
//! HTML, readable with JavaScript off. The card is presentation over the
//! same data and owns no identity of its own.

use leptos::prelude::*;

include!(concat!(env!("OUT_DIR"), "/cv_data.rs"));

// Monochrome service marks from simple-icons 15.16.1 — the same
// copies the footer social icons carry — keyed by cv.toml label in
// `identity_icon` below and rendered in Cherenkov via currentColor,
// the small-size treatment of the Standards chapter icons.
const ICON_CLARIVATE: &str = "M11.262 18a20.588 20.588 0 0 0 3.246-1.476 20.954 20.954 0 0 0 2.852-2.064A14.46 14.46 0 0 0 19.82 12c.328.328.622.688.884 1.082.328.394.592.786.788 1.18.262.46.524.886.786 1.28.198.458.394.884.59 1.278a23.172 23.172 0 0 1-2.654 2.46 38.765 38.765 0 0 1-2.952 1.966A29.612 29.612 0 0 1 10.574 24l-.788-1.18c-.26-.46-.524-.886-.786-1.28-.209-.42-.406-.846-.59-1.278l-.59-1.376A20.4 20.4 0 0 0 11.26 18h.002zm2.754-16.918a29.61 29.61 0 0 1 3.246 1.574c.984.59 1.968 1.246 2.952 1.966a47.566 47.566 0 0 1 2.654 2.46c-.196.46-.392.886-.59 1.28-.26.458-.524.884-.786 1.278a6.16 6.16 0 0 1-.788 1.18c-.273.409-.568.803-.884 1.18a18.072 18.072 0 0 0-2.46-2.558 20.936 20.936 0 0 0-2.852-2.064 20.4 20.4 0 0 0-6.688-2.36l.59-1.38c.196-.392.394-.82.59-1.278l.786-1.18.788-1.18c1.18.262 2.328.622 3.442 1.082zm-7.574 18c-.524.066-1.016.1-1.474.1H3.492c-.46 0-.92-.034-1.378-.1a34.28 34.28 0 0 1-.786-3.54 32.942 32.942 0 0 1 0-7.18c.196-1.18.46-2.33.786-3.444l1.378-.196h1.476c.46 0 .95.032 1.474.098l1.378.196-.59 1.77c-.262.984-.46 2-.59 3.05-.066.984-.1 2-.1 3.05.066 1.048.198 2.064.394 3.048.262 1.05.558 2.034.886 2.952-.46.13-.92.196-1.378.196z";
const ICON_GITHUB: &str = "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12";
const ICON_GITLAB: &str = "m23.6004 9.5927-.0337-.0862L20.3.9814a.851.851 0 0 0-.3362-.405.8748.8748 0 0 0-.9997.0539.8748.8748 0 0 0-.29.4399l-2.2055 6.748H7.5375l-2.2057-6.748a.8573.8573 0 0 0-.29-.4412.8748.8748 0 0 0-.9997-.0537.8585.8585 0 0 0-.3362.4049L.4332 9.5015l-.0325.0862a6.0657 6.0657 0 0 0 2.0119 7.0105l.0113.0087.03.0213 4.976 3.7264 2.462 1.8633 1.4995 1.1321a1.0085 1.0085 0 0 0 1.2197 0l1.4995-1.1321 2.4619-1.8633 5.006-3.7489.0125-.01a6.0682 6.0682 0 0 0 2.0094-7.003z";
const ICON_GOOGLE_PLAY: &str = "M22.018 13.298l-3.919 2.218-3.515-3.493 3.543-3.521 3.891 2.202a1.49 1.49 0 0 1 0 2.594zM1.337.924a1.486 1.486 0 0 0-.112.568v21.017c0 .217.045.419.124.6l11.155-11.087L1.337.924zm12.207 10.065l3.258-3.238L3.45.195a1.466 1.466 0 0 0-.946-.179l11.04 10.973zm0 2.067l-11 10.933c.298.036.612-.016.906-.183l13.324-7.54-3.23-3.21z";
const ICON_HACKERONE: &str = "M7.207 0c-.4836 0-.8774.1018-1.1823.3002-.3044.2003-.4592.4627-.4592.7798v21.809c0 .2766.1581.5277.4752.7609.315.2335.7031.3501 1.1664.3501.4427 0 .8306-.1166 1.1678-.3501.3352-.231.5058-.4843.5058-.761V1.0815c0-.319-.1623-.5769-.4893-.7813C8.0644.1018 7.6702 0 7.207 0zm9.5234 8.662c-.4836 0-.8717.0981-1.1683.3007l-4.439 2.7822c-.1988.1861-.2841.4687-.2473.855.0342.3826.2108.747.5238 1.0907.3145.346.6662.5626 1.0684.6547.3963.0899.6973.041.8962-.143l1.7551-1.0951v9.7817c0 .2767.1522.5278.4607.761.3007.2335.6873.3501 1.1504.3501.463 0 .863-.1166 1.1983-.3501.3371-.2332.5058-.4843.5058-.761V9.7381c0-.3193-.165-.577-.4898-.7754-.3252-.2026-.7288-.3007-1.2143-.3007z";
const ICON_HUGGINGFACE: &str = "M12.025 1.13c-5.77 0-10.449 4.647-10.449 10.378 0 1.112.178 2.181.503 3.185.064-.222.203-.444.416-.577a.96.96 0 0 1 .524-.15c.293 0 .584.124.84.284.278.173.48.408.71.694.226.282.458.611.684.951v-.014c.017-.324.106-.622.264-.874s.403-.487.762-.543c.3-.047.596.06.787.203s.31.313.4.467c.15.257.212.468.233.542.01.026.653 1.552 1.657 2.54.616.605 1.01 1.223 1.082 1.912.055.537-.096 1.059-.38 1.572.637.121 1.294.187 1.967.187.657 0 1.298-.063 1.921-.178-.287-.517-.44-1.041-.384-1.581.07-.69.465-1.307 1.081-1.913 1.004-.987 1.647-2.513 1.657-2.539.021-.074.083-.285.233-.542.09-.154.208-.323.4-.467a1.08 1.08 0 0 1 .787-.203c.359.056.604.29.762.543s.247.55.265.874v.015c.225-.34.457-.67.683-.952.23-.286.432-.52.71-.694.257-.16.547-.284.84-.285a.97.97 0 0 1 .524.151c.228.143.373.388.43.625l.006.04a10.3 10.3 0 0 0 .534-3.273c0-5.731-4.678-10.378-10.449-10.378M8.327 6.583a1.5 1.5 0 0 1 .713.174 1.487 1.487 0 0 1 .617 2.013c-.183.343-.762-.214-1.102-.094-.38.134-.532.914-.917.71a1.487 1.487 0 0 1 .69-2.803m7.486 0a1.487 1.487 0 0 1 .689 2.803c-.385.204-.536-.576-.916-.71-.34-.12-.92.437-1.103.094a1.487 1.487 0 0 1 .617-2.013 1.5 1.5 0 0 1 .713-.174m-10.68 1.55a.96.96 0 1 1 0 1.921.96.96 0 0 1 0-1.92m13.838 0a.96.96 0 1 1 0 1.92.96.96 0 0 1 0-1.92M8.489 11.458c.588.01 1.965 1.157 3.572 1.164 1.607-.007 2.984-1.155 3.572-1.164.196-.003.305.12.305.454 0 .886-.424 2.328-1.563 3.202-.22-.756-1.396-1.366-1.63-1.32q-.011.001-.02.006l-.044.026-.01.008-.03.024q-.018.017-.035.036l-.032.04a1 1 0 0 0-.058.09l-.014.025q-.049.088-.11.19a1 1 0 0 1-.083.116 1.2 1.2 0 0 1-.173.18q-.035.029-.075.058a1.3 1.3 0 0 1-.251-.243 1 1 0 0 1-.076-.107c-.124-.193-.177-.363-.337-.444-.034-.016-.104-.008-.2.022q-.094.03-.216.087-.06.028-.125.063l-.13.074q-.067.04-.136.086a3 3 0 0 0-.135.096 3 3 0 0 0-.26.219 2 2 0 0 0-.12.121 2 2 0 0 0-.106.128l-.002.002a2 2 0 0 0-.09.132l-.001.001a1.2 1.2 0 0 0-.105.212q-.013.036-.024.073c-1.139-.875-1.563-2.317-1.563-3.203 0-.334.109-.457.305-.454m.836 10.354c.824-1.19.766-2.082-.365-3.194-1.13-1.112-1.789-2.738-1.789-2.738s-.246-.945-.806-.858-.97 1.499.202 2.362c1.173.864-.233 1.45-.685.64-.45-.812-1.683-2.896-2.322-3.295s-1.089-.175-.938.647 2.822 2.813 2.562 3.244-1.176-.506-1.176-.506-2.866-2.567-3.49-1.898.473 1.23 2.037 2.16c1.564.932 1.686 1.178 1.464 1.53s-3.675-2.511-4-1.297c-.323 1.214 3.524 1.567 3.287 2.405-.238.839-2.71-1.587-3.216-.642-.506.946 3.49 2.056 3.522 2.064 1.29.33 4.568 1.028 5.713-.624m5.349 0c-.824-1.19-.766-2.082.365-3.194 1.13-1.112 1.789-2.738 1.789-2.738s.246-.945.806-.858.97 1.499-.202 2.362c-1.173.864.233 1.45.685.64.451-.812 1.683-2.896 2.322-3.295s1.089-.175.938.647-2.822 2.813-2.562 3.244 1.176-.506 1.176-.506 2.866-2.567 3.49-1.898-.473 1.23-2.037 2.16c-1.564.932-1.686 1.178-1.464 1.53s3.675-2.511 4-1.297c.323 1.214-3.524 1.567-3.287 2.405.238.839 2.71-1.587 3.216-.642.506.946-3.49 2.056-3.522 2.064-1.29.33-4.568 1.028-5.713-.624";
const ICON_KAGGLE: &str = "M.1025 7.3475c-.0681 0-.1022.0341-.1022.102v6.752c0 .0681.034.1022.1022.1022h.7049c.068 0 .1022-.034.1022-.1023v-1.481l.4187-.3985 1.5016 1.91c.041.0477.0884.0716.143.0716h.9091c.0476 0 .0748-.0135.0817-.0407.0135-.041.0066-.075-.0206-.1023l-1.9816-2.4618 1.9002-1.8384c.0204-.0205.0237-.051.01-.092-.0137-.0339-.0408-.051-.0816-.051h-.9398c-.0477 0-.0953.024-.143.0716L.9096 11.607V7.4496c0-.0679-.0342-.102-.1022-.102zm18.0417 0c-.068 0-.102.0341-.102.102v6.752c0 .0681.034.102.102.102h.705c.068 0 .102-.034.102-.102v-6.752c0-.068-.034-.102-.102-.102zM5.961 9.6254c-.5653 0-1.11.1806-1.6343.5415-.0545.0545-.0648.102-.0307.143l.3676.5208c.0272.0477.0717.0545.133.0204.3948-.2722.783-.4086 1.1644-.4086.2927 0 .5158.0886.669.2656.1532.1771.2197.3917.1992.6436-.6606.0681-1.1545.1495-1.4813.245-.8308.2383-1.2461.6913-1.2461 1.3586 0 .4222.1533.7695.4598 1.0419.3132.2654.6845.3982 1.1134.3982.4698 0 .8545-.1125 1.1542-.3372v.1432c0 .0682.0374.102.1123.102h.7048c.068 0 .102-.0338.102-.102V11.372c0-.6604-.2245-1.1406-.6739-1.4403-.3065-.2043-.6776-.3063-1.1134-.3063zm4.3225 0c-.6742 0-1.195.2622-1.5627.7865-.3133.4359-.4699.9671-.4699 1.5936 0 .6604.1634 1.2087.4903 1.6444.3744.4972.892.7455 1.5526.7455.5313 0 .9567-.1327 1.2768-.3982v.531c0 .858-.4122 1.287-1.236 1.287-.361 0-.732-.1907-1.1132-.572a.098.098 0 00-.0716-.0306c-.034 0-.0613.0102-.0817.0307l-.4802.48c-.0408.0613-.0375.1124.0103.1532.1361.1157.2554.2129.3576.2911.102.0783.1905.1413.2656.189.354.1975.7284.2961 1.1235.2961.6808 0 1.207-.1925 1.5781-.577.3711-.3848.5567-.9484.5567-1.6903V9.8196c0-.068-.034-.102-.102-.102h-.705c-.0682 0-.1021.034-.1021.102v.2043c-.3471-.2657-.7763-.3985-1.287-.3985zm4.8021 0c-.6742 0-1.195.2622-1.5627.7865-.3132.4359-.4699.9671-.4699 1.5936 0 .6604.1633 1.2087.4903 1.6444.3744.4972.892.7455 1.5526.7455.5311 0 .9566-.1327 1.2768-.3982v.531c0 .858-.4122 1.287-1.236 1.287-.361 0-.732-.1907-1.1133-.572a.098.098 0 00-.0716-.0306c-.034 0-.0612.0102-.0816.0307l-.48.48c-.0409.0613-.0376.1124.01.1532.1363.1157.2555.2129.3576.2911.1021.0783.1906.1413.2657.189.354.1975.7285.2961 1.1237.2961.6808 0 1.2068-.1925 1.5781-.577.371-.3848.5565-.9484.5565-1.6903V9.8196c0-.068-.034-.102-.102-.102h-.7049c-.0682 0-.1022.034-.1022.102v.2043c-.3474-.2657-.7763-.3985-1.287-.3985zm6.7457 0c-.6537 0-1.185.211-1.5936.6332-.4427.4632-.664 1.0283-.664 1.6956 0 .7083.225 1.2905.6743 1.7467.463.463 1.042.6945 1.7366.6945.6467 0 1.2154-.1838 1.7057-.5515.0545-.041.0545-.0884 0-.143l-.4802-.4903c-.041-.0409-.0919-.0409-.1533 0-.2998.2112-.6368.3167-1.0112.3167-.4222 0-.7729-.119-1.052-.3576-.2452-.2248-.3882-.5038-.429-.8375h3.3197c.0679 0 .1022-.0341.1022-.1023l.01-.2244c.0341-.6878-.1668-1.26-.6025-1.7162-.4224-.4426-.9432-.664-1.5627-.664zm-.0206.7865c.3268 0 .6062.1056.8377.3166.2452.211.371.4734.378.7865h-2.4618c.0613-.3269.2077-.5925.4392-.7968.2313-.2042.5004-.3063.8069-.3063zm-11.4249.102c.6196 0 1.0146.2181 1.1848.6538v1.6854c-.1702.4358-.5755.6538-1.2155.6538-.3133 0-.5687-.0986-.7661-.2963-.2656-.2518-.3983-.6538-.3983-1.2053 0-.9941.3984-1.4914 1.1951-1.4914zm4.802 0c.6196 0 1.0148.2181 1.1851.6538h-.0002v1.6854c-.1703.4358-.5755.6538-1.2155.6538-.3132 0-.5686-.0986-.7661-.2963-.2655-.2518-.3983-.6538-.3983-1.2053 0-.9941.3983-1.4914 1.195-1.4914zm-8.3586 1.6547v1.0215c-.286.286-.6675.412-1.1441.3779-.1703-.0135-.32-.0663-.4493-.1582-.1294-.0919-.2045-.2129-.2249-.3627-.0341-.2657.1158-.47.4495-.6129.2452-.1088.7013-.1974 1.3688-.2656z";
const ICON_ORCID: &str = "M12 0C5.372 0 0 5.372 0 12s5.372 12 12 12 12-5.372 12-12S18.628 0 12 0zM7.369 4.378c.525 0 .947.431.947.947s-.422.947-.947.947a.95.95 0 0 1-.947-.947c0-.525.422-.947.947-.947zm-.722 3.038h1.444v10.041H6.647V7.416zm3.562 0h3.9c3.712 0 5.344 2.653 5.344 5.025 0 2.578-2.016 5.025-5.325 5.025h-3.919V7.416zm1.444 1.303v7.444h2.297c3.272 0 4.022-2.484 4.022-3.722 0-2.016-1.284-3.722-4.097-3.722h-2.222z";
const ICON_RESEARCHGATE: &str = "M19.586 0c-.818 0-1.508.19-2.073.565-.563.377-.97.936-1.213 1.68a3.193 3.193 0 0 0-.112.437 8.365 8.365 0 0 0-.078.53 9 9 0 0 0-.05.727c-.01.282-.013.621-.013 1.016a31.121 31.123 0 0 0 .014 1.017 9 9 0 0 0 .05.727 7.946 7.946 0 0 0 .077.53h-.005a3.334 3.334 0 0 0 .113.438c.245.743.65 1.303 1.214 1.68.565.376 1.256.564 2.075.564.8 0 1.536-.213 2.105-.603.57-.39.94-.916 1.175-1.65.076-.235.135-.558.177-.93a10.9 10.9 0 0 0 .043-1.207v-.82c0-.095-.047-.142-.14-.142h-3.064c-.094 0-.14.047-.14.141v.956c0 .094.046.14.14.14h1.666c.056 0 .084.03.084.086 0 .36 0 .62-.036.865-.038.244-.1.447-.147.606-.108.385-.348.664-.638.876-.29.212-.738.35-1.227.35-.545 0-.901-.15-1.21-.353-.306-.203-.517-.454-.67-.915a3.136 3.136 0 0 1-.147-.762 17.366 17.367 0 0 1-.034-.656c-.01-.26-.014-.572-.014-.939a26.401 26.403 0 0 1 .014-.938 15.821 15.822 0 0 1 .035-.656 3.19 3.19 0 0 1 .148-.76 1.89 1.89 0 0 1 .742-1.01c.344-.244.593-.352 1.137-.352.508 0 .815.096 1.144.303.33.207.528.492.764.925.047.094.111.118.198.07l1.044-.43c.075-.048.09-.115.042-.199a3.549 3.549 0 0 0-.466-.742 3 3 0 0 0-.679-.607 3.313 3.313 0 0 0-.903-.41A4.068 4.068 0 0 0 19.586 0zM8.217 5.836c-1.69 0-3.036.086-4.297.086-1.146 0-2.291 0-3.007-.029v.831l1.088.2c.744.144 1.174.488 1.174 2.264v11.288c0 1.777-.43 2.12-1.174 2.263l-1.088.2v.832c.773-.029 2.12-.086 3.465-.086 1.29 0 2.951.057 3.667.086v-.831l-1.49-.2c-.773-.115-1.174-.487-1.174-2.264v-4.784c.688.057 1.29.057 2.206.057 1.748 3.123 3.41 5.472 4.355 6.56.86 1.032 2.177 1.691 3.839 1.691.487 0 1.003-.086 1.318-.23v-.744c-1.031 0-2.063-.716-2.808-1.518-1.26-1.376-2.95-3.582-4.355-6.074 2.32-.545 4.04-2.722 4.04-4.9 0-3.208-2.492-4.698-5.758-4.698zm-.515 1.29c2.406 0 3.839 1.26 3.839 3.552 0 2.263-1.547 3.782-4.097 3.782-.974 0-1.404-.03-2.063-.086v-7.19c.66-.059 1.547-.059 2.32-.059z";
const ICON_SALESFORCE: &str = "M10.006 5.415a4.195 4.195 0 013.045-1.306c1.56 0 2.954.9 3.69 2.205.63-.3 1.35-.45 2.1-.45 2.85 0 5.159 2.34 5.159 5.22s-2.31 5.22-5.176 5.22c-.345 0-.69-.044-1.02-.104a3.75 3.75 0 01-3.3 1.95c-.6 0-1.155-.15-1.65-.375A4.314 4.314 0 018.88 20.4a4.302 4.302 0 01-4.05-2.82c-.27.062-.54.076-.825.076-2.204 0-4.005-1.8-4.005-4.05 0-1.5.811-2.805 2.01-3.51-.255-.57-.39-1.2-.39-1.846 0-2.58 2.1-4.65 4.65-4.65 1.53 0 2.85.705 3.72 1.8";
const ICON_SCOPUS: &str = "M24 19.059l-.14-1.777c-1.426.772-2.945 1.076-4.465 1.076-3.319 0-5.96-2.782-5.96-6.475 0-3.903 2.595-6.31 5.633-6.31 1.917 0 3.39.303 4.792 1.075L24 4.895c-1.286-.608-2.337-.889-4.698-.889-4.534 0-7.97 3.53-7.97 8.017 0 5.12 4.09 7.924 7.9 7.924 1.916 0 3.506-.257 4.768-.888zm-14.954-3.46c0-2.22-1.964-3.225-3.857-4.347C3.716 10.364 2.15 9.756 2.15 8.12c0-1.215.889-2.548 2.642-2.548 1.519 0 2.57.234 3.903 1.029l.117-1.847c-1.239-.514-2.127-.748-4.137-.748C1.8 4.006.047 5.876.047 8.26c0 2.384 2.103 3.413 4.02 4.581 1.426.865 2.922 1.45 2.922 2.992 0 1.496-1.333 2.571-2.922 2.571-1.566 0-2.594-.35-3.786-1.075L0 19.176c1.215.56 2.454.818 4.16.818 2.385 0 4.885-1.473 4.885-4.395z";
const ICON_YOUTUBE: &str = "M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z";

const ICON_ZENODO: &str = "M23.82698 11.163a2.2419 2.2419 0 00-.46698-.68697 2.1499 2.1499 0 00-1.52393-.62697 2.10991 2.10991 0 00-1.20695.36499 2.2469 2.2469 0 00-.49498.45398c-.06.075-.125.17499-.163.23599a2.68689 2.68689 0 00-.20498.46398c.006-.27.046-.67197.20499-.89797V8.52212a.42698.42698 0 00-.137-.32199.44298.44298 0 00-.31698-.132.43998.43998 0 00-.32299.132.43998.43998 0 00-.13.32199v1.73292a2.3699 2.3699 0 00-.59197-.29898 2.1239 2.1239 0 00-1.49994.06 2.1599 2.1599 0 00-1.15295 1.14695l-.024.06a2.2419 2.2419 0 00-.48997-.74697 2.1529 2.1529 0 00-1.52394-.62698 2.10991 2.10991 0 00-.84296.167 2.2299 2.2299 0 00-.68697.45998 2.10591 2.10591 0 00-.46598.68697l-.023.056-.023-.056a2.1549 2.1549 0 00-1.98992-1.31395c-.30098 0-.58197.056-.84196.167a2.2299 2.2299 0 00-.68597.45998c-.2.19599-.35499.42398-.46698.68697l-.02.053-.022-.053a2.2429 2.2429 0 00-.46698-.68698 2.1479 2.1479 0 00-1.52293-.62697 2.10991 2.10991 0 00-.84197.167 2.2289 2.2289 0 00-.64197.41798v-.072a.46298.46298 0 00-.46198-.46298H.53398a.46298.46298 0 000 .92596h2.54889L.091 14.84985a.46198.46198 0 00-.091.27499v.30198c0 .256.207.46298.46298.46298h3.44985a.46198.46198 0 00.46198-.42998 2.1689 2.1689 0 001.34695.47198c.44598 0 .84396-.12 1.19494-.35998.283-.192.50698-.43098.66998-.71997v.62597c0 .127.043.235.13.32199a.43598.43598 0 00.32398.132c.12 0 .22399-.044.31598-.132a.42698.42698 0 00.138-.32199V13.8579h-.002l.002-.003H7.36069a.43998.43998 0 00-.41799.28699l-.074.152a1.23395 1.23395 0 01-.45997.53197 1.23395 1.23395 0 01-.68697.197c-.168 0-.32999-.031-.48498-.095a1.17495 1.17495 0 01-.39999-.27 1.38194 1.38194 0 01-.26998-.39998 1.16495 1.16495 0 01-.1-.47698v-.40598h4.02783v-1.37494a1.23695 1.23695 0 01.36498-.87796 1.23995 1.23995 0 01.88996-.36499c.167 0 .32899.032.48398.095.155.065.289.154.39999.27.112.11499.19999.24898.26898.39997.067.151.102.311.102.47798v3.47686c0 .12799.043.23599.13.32298a.43798.43798 0 00.32398.132c.12 0 .224-.044.31599-.132a.43098.43098 0 00.137-.32299v-.62297a2.1439 2.1439 0 001.86791 1.07795c.29399 0 .57398-.056.83697-.168.26199-.11099.49298-.26398.69297-.45997a2.11191 2.11191 0 00.48398-.73297l.018.047c.111.26199.26699.48998.46598.68697.19999.195.42798.34799.68697.45998.25799.11.53997.167.84296.167a2.1469 2.1469 0 001.52294-.62698c.19499-.196.35098-.42498.46698-.68697l.024-.06.024.06c.115.26199.26998.48998.46598.68697a2.1439 2.1439 0 001.52393.62697c.29398 0 .57397-.056.83696-.168a2.2199 2.2199 0 00.69297-.45997 2.11691 2.11691 0 00.63498-1.52194v-1.78192a2.07991 2.07991 0 00-.173-.83696zM1.15995 14.96485l2.4199-3.25886c-.012.096-.02.194-.02.29399v1.77992c0 .296.058.57398.174.83797.05399.123.11799.23899.18998.34698zm5.81775-2.49989H4.4678v-.46498a1.24195 1.24195 0 01.76498-1.14695c.15199-.064.31498-.096.48998-.096.16799 0 .32998.032.48398.095.15599.065.28998.154.39998.26999.112.115.19999.24899.26999.39998.068.151.1.31099.1.47798zm8.05766 1.31594a1.24895 1.24895 0 01-1.25395 1.24295c-.168 0-.32898-.032-.48498-.096a1.16795 1.16795 0 01-.39998-.26999 1.37594 1.37594 0 01-.26899-.39998 1.15895 1.15895 0 01-.1-.47698v-1.78092a1.23995 1.23995 0 011.25495-1.24295c.16599 0 .32798.032.48198.095.15599.065.28998.154.39998.26999.112.115.201.24899.26999.39998.067.152.101.31099.101.47798zm4.02783 0c0 .167-.033.32599-.1.47798a1.37994 1.37994 0 01-.27.39999 1.15995 1.15995 0 01-.39998.26998 1.25995 1.25995 0 01-.48397.094 1.23195 1.23195 0 01-.88997-.36398 1.24195 1.24195 0 01-.36598-.87796v-1.78093c0-.16699.035-.32598.102-.47798a1.39994 1.39994 0 01.26998-.39998 1.15995 1.15995 0 01.39999-.26999 1.26995 1.26995 0 01.48398-.095 1.24995 1.24995 0 01.88996.36499 1.24995 1.24995 0 01.36398.87796zm4.02782 0a1.22395 1.22395 0 01-.36398.87797c-.116.116-.24999.20499-.39998.26998a1.24995 1.24995 0 01-.48998.094 1.26995 1.26995 0 01-.48498-.095 1.16895 1.16895 0 01-.39999-.26998 1.37994 1.37994 0 01-.26998-.39999 1.15695 1.15695 0 01-.1-.47698v-1.78092a1.23995 1.23995 0 011.25494-1.24295c.167 0 .32799.032.48298.095.156.065.29.154.39999.27.112.11499.20099.24898.26998.39997.067.152.1.311.1.47798v1.77993z";

/// The recognized mark for one identity, keyed by its cv.toml label.
///
/// ORCID, GitHub, GitLab, Google Play, `HackerOne`, Hugging Face, Kaggle,
/// `ResearchGate`, Salesforce (for Trailhead), `YouTube`, Scopus, and Zenodo
/// use their simple-icons paths; the Web of Science `ResearcherID` uses
/// Clarivate's mark (Web of Science is Clarivate's service and has
/// no mark of its own there). The rest have no simple-icons mark,
/// so they carry each service's own favicon, self-hosted under
/// public/img/icons/: Frontiers' concentric-target glyph for
/// Loop, MDPI's "Sci" bubble for `SciProfiles`, Authenticus' "a"
/// ring, Doximity's arcs-and-"d", the Knowledge Commons swirl
/// hcommons.org serves for Humanities Commons, the Microsoft
/// four-squares learn.microsoft.com serves for Microsoft Learn
/// (simple-icons carries neither `microsoftlearn` nor
/// `microsoft`), and the state-profile favicon
/// secure.lni.wa.gov serves for `Washington L&I`.
/// `OpenAlex` has no simple-icons mark either: its
/// icon is the tricon openalex.org itself serves, tinted
/// Cherenkov like the paths above — the served original is a
/// black silhouette that would vanish on the abyss ground.
/// `OpenAIRE` has no simple-icons mark either: its icon is
/// the ring-and-plus glyph explore.openaire.eu itself
/// serves, tinted Cherenkov the same way — the served
/// original sits on an opaque white square and its navy
/// ring is too dark for the abyss ground.
/// crates.io and `LuaRocks` have no simple-icons mark
/// either (15.16.1 carries neither `cratesio` nor
/// `luarocks`): each carries the favicon.ico its own site
/// serves, kept byte for byte as fetched on 2026-10-13.
/// Every icon is decorative — the label beside it
/// carries the name.
fn identity_icon(label: &str) -> AnyView {
    match label {
        "AuthenticusID" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/authenticus.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "crates.io" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/crates-io.ico"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "Doximity" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/doximity.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "Humanities Commons" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/hcommons.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "Loop profile" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/loop.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "LuaRocks" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/luarocks.ico"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "Microsoft Learn" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/microsoft-learn.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "OpenAIRE" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/openaire.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "OpenAlex" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/openalex.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "SciProfiles" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/sciprofiles.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        "Washington L&I" => view! {
            <img
                class="cv-id-icon"
                src="/img/icons/washington-lni.png"
                alt=""
                aria-hidden="true"
            />
        }
        .into_any(),
        _ => {
            let path = match label {
                "GitHub" => ICON_GITHUB,
                "GitLab" => ICON_GITLAB,
                "Google Play" => ICON_GOOGLE_PLAY,
                "HackerOne" => ICON_HACKERONE,
                "Hugging Face" => ICON_HUGGINGFACE,
                "Kaggle" => ICON_KAGGLE,
                "ORCID" => ICON_ORCID,
                "ResearchGate" => ICON_RESEARCHGATE,
                "ResearcherID (Web of Science)" => ICON_CLARIVATE,
                "Salesforce Trailhead" => ICON_SALESFORCE,
                "Scopus Author ID" => ICON_SCOPUS,
                "YouTube" => ICON_YOUTUBE,
                "Zenodo" => ICON_ZENODO,
                _ => "",
            };
            view! {
                <svg
                    class="cv-id-icon"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path d={path} />
                </svg>
            }
            .into_any()
        }
    }
}

/// The identity the card heads to before anyone touches it — the
/// persistent researcher identifier. Falls back to the first entry if
/// cv.toml ever drops it.
const HOME_LABEL: &str = "ORCID";

/// One drift step: every period the card heads to the next identity.
/// The glide between headings is the CSS transition, not this timer.
#[cfg(feature = "hydrate")]
const DRIFT_PERIOD: std::time::Duration = std::time::Duration::from_secs(6);

/// Longest query the search matches against. Longer input is cut, so
/// one keystroke costs at most every identity times three fields of
/// this many characters.
const QUERY_MAX: usize = 64;

/// How well `id` matches a normalized `query`; lower is better.
///
/// `None` is no match. Exact name, exact value, name prefix, name
/// substring, value substring, then the kind line — so "orcid" lands
/// on ORCID and "license" on the state license.
fn match_rank(id: &CvIdentifier, query: &str) -> Option<u8> {
    if query.is_empty() {
        return None;
    }
    let label = id.label.to_lowercase();
    let value = id.value.to_lowercase();
    if label == query {
        Some(0)
    } else if value == query {
        Some(1)
    } else if label.starts_with(query) {
        Some(2)
    } else if label.contains(query) {
        Some(3)
    } else if value.contains(query) {
        Some(4)
    } else if id.kind.to_lowercase().contains(query) {
        Some(5)
    } else {
        None
    }
}

/// Search input as matched: trimmed, capped at `QUERY_MAX` characters,
/// lowercased.
fn normalize_query(raw: &str) -> String {
    raw.trim().chars().take(QUERY_MAX).collect::<String>().to_lowercase()
}

/// Index of the best match for a normalized `query` — lowest rank,
/// ties to the earlier entry — or `None` when nothing matches.
fn best_match(ids: &[CvIdentifier], query: &str) -> Option<usize> {
    ids.iter().enumerate().filter_map(|(i, id)| match_rank(id, query).map(|rank| (rank, i))).min().map(|(_, i)| i)
}

/// Signed tick count from position `from` to position `to` on a card
/// of `n` ticks, the short way round (a half-turn tie goes forward).
/// Both positions must lie in `0..n`; anything else turns nothing.
fn shortest_steps(from: i32, to: i32, n: i32) -> i32 {
    if n <= 0 || !(0..n).contains(&from) || !(0..n).contains(&to) {
        return 0;
    }
    let ahead = (to - from).rem_euclid(n);
    if ahead > n / 2 {
        ahead - n
    } else {
        ahead
    }
}

/// The text drawn on a tick: the label up to any parenthetical, so
/// `ResearcherID (Web of Science)` reads `ResearcherID` on the card.
/// The readout and the list keep the full label.
fn tick_text(label: &str) -> &str {
    label.split_once(" (").map_or(label, |(head, _)| head)
}

fn tick_class(heading: bool, matched: bool) -> &'static str {
    if heading {
        "ring-tick is-heading"
    } else if matched {
        "ring-tick is-match"
    } else {
        "ring-tick"
    }
}

/// List entries dim — never leave the DOM — while a search is active,
/// so the prerendered list stays complete for find-in-page.
fn list_class(heading: bool, searching: bool, matched: bool) -> &'static str {
    if heading {
        "cv-id is-heading"
    } else if !searching {
        "cv-id"
    } else if matched {
        "cv-id is-match"
    } else {
        "cv-id is-dim"
    }
}

/// The mailbox, the key, and the identity card over its plain list.
/// Personal presentation: the section speaks for Matt, not a company.
#[component]
pub fn IdentitySection() -> impl IntoView {
    view! {
        <section class="identity" id="identity" aria-labelledby="identity-head">
            <p class="page-bearing">"CHART — OPEN A CHANNEL"</p>
            <h2 class="cv-head" id="identity-head">"Contact"</h2>
            <p class="page-lede">
                "One mailbox, answered by me. Encrypt it if the matter warrants — "
                "the key is published on my own key server."
            </p>
            <div class="entry-list">
                <article class="entry">
                    <p class="entry-kind">"EMAIL"</p>
                    <h3>
                        <a href="mailto:map@qompass.ai">"map@qompass.ai"</a>
                    </h3>
                    <p>"Research, consulting, and collaboration inquiries."</p>
                </article>
                <article class="entry">
                    <p class="entry-kind">"OPENPGP — WEB KEY DIRECTORY"</p>
                    <h3 class="mono-block">"3410 59A8 D566 DE30 B4B6 7FC5 FC89 A70A C651 68FD"</h3>
                    <p>
                        "Fetch it the modern way: " <span class="mono-inline">"gpg --locate-keys map@qompass.ai"</span>
                        " — served by volta at keys.qompass.ai — or verify the fingerprint "
                        "above against any copy you already hold."
                    </p>
                </article>
            </div>
            <IdentityCard />
        </section>
    }
    // Type-erased at the boundary: nested inside the About view, the
    // full static type of this tree overflows the binary crate's
    // default query depth (the lib raises its own limit for the home
    // page; the bin does not).
    .into_any()
}

/// The compass card and the guarantee list, sharing one selection.
///
/// Every piece of state is a signal named here:
/// - `selected` — the identity in the readout; always a valid index,
///   because every change goes through `select`.
/// - `turn` — the card's angle in whole ticks. `select` moves it the
///   short way to `selected` and the CSS transition glides it there;
///   nothing reads the angle back, so the readout can never show a
///   half-turned or stale heading.
/// - `held` — the visitor stopped the drift (a tap, a tick, a search,
///   or the pause button); `hovering` pauses it only while the pointer
///   or keyboard focus is on the card.
/// - `hydrated`, `drifting` — false in the prerender. The search box,
///   copy button, and tick focus stops switch on after hydration; the
///   drift starts only when the visitor has not asked for reduced
///   motion.
#[component]
fn IdentityCard() -> impl IntoView {
    let ids = CV_IDENTIFIERS;
    let len = ids.len();
    // build.rs refuses a cv.toml without identifiers, so `ticks` >= 1
    // from the data; the max(1) keeps rem_euclid total regardless.
    let ticks = i32::try_from(len).unwrap_or(i32::MAX).max(1);
    let home = ids.iter().position(|id| id.label == HOME_LABEL).unwrap_or(0);
    let home_turn = i32::try_from(home).unwrap_or(0);
    let tick_deg = 360.0 / f64::from(ticks);

    let selected = RwSignal::new(home);
    let turn = RwSignal::new(home_turn);
    let query = RwSignal::new(String::new());
    let status = RwSignal::new(String::new());
    let held = RwSignal::new(false);
    let hovering = RwSignal::new(false);
    let hydrated = RwSignal::new(false);
    let drifting = RwSignal::new(false);
    let copied = RwSignal::new(false);

    let select = move |i: usize| {
        let Ok(target) = i32::try_from(i) else {
            return;
        };
        if i >= len {
            return;
        }
        // Saturating: at one tick per drift period the counter needs
        // centuries to get near i32::MAX.
        turn.update(|t| *t = t.saturating_add(shortest_steps(t.rem_euclid(ticks), target, ticks)));
        selected.set(i);
        copied.set(false);
    };

    Effect::new(move |_| {
        hydrated.set(true);
        #[cfg(feature = "hydrate")]
        start_drift(drifting, held, hovering, selected, select, len);
    });

    let copy = move |_| {
        #[cfg(feature = "hydrate")]
        copy_value(selected.get_untracked(), copied);
    };

    let on_search = move |ev| {
        let q = normalize_query(&leptos::leptos_dom::helpers::event_target_value(&ev));
        if q.is_empty() {
            status.set(String::new());
        } else {
            held.set(true);
            let message = best_match(ids, &q).map_or_else(
                || format!("No identity matches \u{201c}{q}\u{201d}."),
                |i| {
                    select(i);
                    ids.get(i).map_or_else(String::new, |id| format!("Heading: {} — {}", id.label, id.value))
                },
            );
            status.set(message);
        }
        query.set(q);
    };

    let card_style = move || {
        let home_deg = -f64::from(home_turn) * tick_deg;
        let turn_deg = -f64::from(turn.get()) * tick_deg;
        format!("--ring-home: {home_deg:.3}deg; --ring-turn: {turn_deg:.3}deg")
    };

    let ticks_view = ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let at = f64::from(i32::try_from(i).unwrap_or(0)) * tick_deg;
            let between = at + tick_deg / 2.0;
            view! {
                <line
                    class="ring-minor"
                    x1="300"
                    y1="14"
                    x2="300"
                    y2="26"
                    transform=format!("rotate({between:.3} 300 300)")
                />
                <g
                    class=move || tick_class(selected.get() == i, query.with(|q| match_rank(id, q).is_some()))
                    transform=format!("rotate({at:.3} 300 300)")
                    role=move || hydrated.get().then_some("button")
                    tabindex=move || hydrated.get().then_some("0")
                    aria-label=id.label
                    aria-pressed=move || hydrated.get().then(|| (selected.get() == i).to_string())
                    on:click=move |_| {
                        held.set(true);
                        select(i);
                    }
                    on:keydown=move |ev| {
                        let key = ev.key();
                        if key == "Enter" || key == " " {
                            ev.prevent_default();
                            held.set(true);
                            select(i);
                        }
                    }
                >
                    <rect class="ring-hit" x="284" y="6" width="32" height="182" rx="4" />
                    <line class="ring-mark" x1="300" y1="10" x2="300" y2="36" />
                    <text class="ring-label" x="300" y="46" transform="rotate(90 300 46)">
                        {tick_text(id.label)}
                    </text>
                </g>
            }
        })
        .collect_view();

    let readout = move || {
        ids.get(selected.get()).map(|id| {
            view! {
                <p class="ring-kind">{id.kind}</p>
                <p class="ring-name">{identity_icon(id.label)} <span>{id.label}</span></p>
                <p class="ring-value">{id.value}</p>
                <p class="ring-actions">
                    <button type="button" class="ring-copy" disabled=move || !hydrated.get() on:click=copy>
                        {move || if copied.get() { "Copied" } else { "Copy" }}
                    </button>
                    <a class="ring-open" href=id.url aria-label=format!("Open {}", id.label)>
                        "Open"
                    </a>
                </p>
            }
        })
    };

    view! {
        <div class="ring-wrap">
            <div class="ring-controls">
                <label class="ring-search">
                    <span>"Search identities"</span>
                    <input
                        type="search"
                        maxlength="64"
                        autocomplete="off"
                        spellcheck="false"
                        placeholder="try “license” or “orcid”"
                        aria-controls="identity-list"
                        disabled=move || !hydrated.get()
                        on:input=on_search
                    />
                </label>
                <Show when=move || drifting.get()>
                    <button type="button" class="ring-toggle" on:click=move |_| held.update(|h| *h = !*h)>
                        {move || if held.get() { "Resume drift" } else { "Pause drift" }}
                    </button>
                </Show>
            </div>
            <p class="ring-status" role="status">
                {move || status.get()}
            </p>
            <div
                class="ring"
                on:mouseenter=move |_| hovering.set(true)
                on:mouseleave=move |_| hovering.set(false)
                on:focusin=move |_| hovering.set(true)
                on:focusout=move |_| hovering.set(false)
                on:pointerdown=move |_| held.set(true)
            >
                <svg class="ring-dial" viewBox="0 0 600 600" role="group" aria-label="Identity compass card">
                    <g class="ring-card" style=card_style>
                        <circle class="ring-rim" cx="300" cy="300" r="292" />
                        <circle class="ring-inner" cx="300" cy="300" r="112" />
                        {ticks_view}
                    </g>
                    <polygon class="ring-lubber" points="290,0 310,0 300,16" />
                    <line class="ring-lubber" x1="300" y1="16" x2="300" y2="40" />
                </svg>
                <div class="ring-readout">{readout}</div>
            </div>
        </div>
        <article class="entry entry-identities">
            <p class="entry-kind">"EVERY IDENTITY, LISTED"</p>
            <h3>{format!("{len} identities")}</h3>
            <ul class="cv-ids" id="identity-list">
                {ids
                    .iter()
                    .enumerate()
                    .map(|(i, id)| {
                        view! {
                            <li
                                class=move || {
                                    query.with(|q| list_class(selected.get() == i, !q.is_empty(), match_rank(id, q).is_some()))
                                }
                                aria-label=id.label
                            >
                                {identity_icon(id.label)}
                                <span class="cv-id-label">{id.label} ": "</span>
                                <a href=id.url>{id.value}</a>
                                <span class="cv-id-kind">{id.kind}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </article>
    }
}

/// Start the drift: one heading step per `DRIFT_PERIOD`.
///
/// A step is skipped while the visitor has held the card or has the
/// pointer or focus on it. Without `matchMedia`, or for a visitor who
/// asked for reduced motion, the drift never starts and the card stays
/// exactly where the prerender drew it.
#[cfg(feature = "hydrate")]
fn start_drift(
    drifting: RwSignal<bool>,
    held: RwSignal<bool>,
    hovering: RwSignal<bool>,
    selected: RwSignal<usize>,
    select: impl Fn(usize) + 'static,
    len: usize,
) {
    let reduced = web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .is_none_or(|m| m.matches());
    if reduced || len < 2 {
        return;
    }
    drifting.set(true);
    let step = move || {
        if held.get_untracked() || hovering.get_untracked() {
            return;
        }
        select(selected.get_untracked().saturating_add(1).checked_rem(len).unwrap_or(0));
    };
    if let Ok(handle) = leptos::leptos_dom::helpers::set_interval_with_handle(step, DRIFT_PERIOD) {
        on_cleanup(move || handle.clear());
    }
}

/// Copy the selected identity's value to the clipboard.
///
/// Same settle-or-swallow handling as the home page's Quickstart copy:
/// the label flips only on a settled write, a denied write is consumed
/// on the derived promise, and `forget()` hands each closure to the JS
/// side, which frees it when called.
#[cfg(feature = "hydrate")]
fn copy_value(index: usize, copied: RwSignal<bool>) {
    let (Some(window), Some(id)) = (web_sys::window(), CV_IDENTIFIERS.get(index)) else {
        return;
    };
    let promise = window.navigator().clipboard().write_text(id.value);
    let mark_copied = wasm_bindgen::closure::Closure::once(move |_| copied.set(true));
    let swallow = wasm_bindgen::closure::Closure::once(|_| ());
    let _ = promise.then(&mark_copied).catch(&swallow);
    mark_copied.forget();
    swallow.forget();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn heading_for(raw: &str) -> Option<&'static str> {
        best_match(CV_IDENTIFIERS, &normalize_query(raw)).and_then(|i| CV_IDENTIFIERS.get(i)).map(|id| id.label)
    }

    #[test]
    fn the_card_carries_all_twenty_four_identities() {
        assert_eq!(CV_IDENTIFIERS.len(), 24);
        assert!(CV_IDENTIFIERS.iter().any(|id| id.label == HOME_LABEL));
    }

    #[test]
    fn search_steers_to_the_named_identity() {
        assert_eq!(heading_for("orcid"), Some("ORCID"));
        assert_eq!(heading_for("  LICENSE "), Some("Washington L&I"));
        assert_eq!(heading_for("7351560589446098345"), Some("Google Play"));
        assert_eq!(heading_for("crates"), Some("crates.io"));
        assert_eq!(heading_for("LuaRocks"), Some("LuaRocks"));
    }

    #[test]
    fn the_card_turns_the_short_way_round() {
        assert_eq!(shortest_steps(23, 0, 24), 1);
        assert_eq!(shortest_steps(0, 23, 24), -1);
        assert_eq!(shortest_steps(5, 5, 24), 0);
        assert_eq!(shortest_steps(0, 12, 24), 12);
    }

    #[test]
    fn every_identity_is_complete_and_unique() {
        for (i, id) in CV_IDENTIFIERS.iter().enumerate() {
            assert!(!id.kind.is_empty() && !id.value.is_empty(), "{}", id.label);
            assert!(id.url.starts_with("https://"), "{}", id.label);
            assert!(CV_IDENTIFIERS.iter().skip(i + 1).all(|other| other.label != id.label), "{}", id.label);
        }
    }

    #[test]
    fn blank_queries_match_nothing() {
        for raw in ["", "   ", "\t\n"] {
            assert_eq!(heading_for(raw), None, "{raw:?}");
        }
    }

    #[test]
    fn hostile_queries_match_nothing_and_stay_bounded() {
        for raw in ["<script>alert(1)</script>", "zzzz-no-such-identity", "\u{0}", "' OR 1=1 --"] {
            assert_eq!(heading_for(raw), None, "{raw:?}");
        }
        let flood = "x".repeat(100_000);
        assert!(normalize_query(&flood).chars().count() <= QUERY_MAX);
        assert_eq!(heading_for(&flood), None);
    }

    #[test]
    fn out_of_range_positions_never_turn_the_card() {
        assert_eq!(shortest_steps(-1, 0, 24), 0);
        assert_eq!(shortest_steps(0, 24, 24), 0);
        assert_eq!(shortest_steps(0, 1, 0), 0);
        assert_eq!(shortest_steps(i32::MIN, i32::MAX, 24), 0);
    }

    #[test]
    fn tick_text_drops_only_the_parenthetical() {
        assert_eq!(tick_text("ResearcherID (Web of Science)"), "ResearcherID");
        assert_eq!(tick_text("ORCID"), "ORCID");
        assert_eq!(tick_text(""), "");
        assert_eq!(tick_text(" ("), "");
    }
}
