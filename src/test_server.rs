use std::net::SocketAddr;

use axum::{routing, Json, Router};
use serde_json::json;
use tracing::info;

pub async fn server() {
    let app = Router::new().route(
        "/server-route-1",
        routing::get(|| async { Json(json! ([
            {
              "_id": "62391b9943d829c33b6ea786",
              "index": 0,
              "guid": "ccbe38a0-a707-4158-ad37-e784492aaf36",
              "isActive": true,
              "balance": "$3,767.08",
              "picture": "http://placehold.it/32x32",
              "age": 35,
              "eyeColor": "brown",
              "name": "Camacho Robles",
              "gender": "male",
              "company": "TROPOLIS",
              "email": "camachorobles@tropolis.com",
              "phone": "+1 (854) 481-2425",
              "address": "377 Miami Court, Norwood, Texas, 4551",
              "about": "Mollit id id mollit consectetur. Et est adipisicing dolor consectetur adipisicing ut occaecat nisi adipisicing do. Cupidatat deserunt excepteur fugiat ut. Eiusmod dolore sit nisi ut et est id adipisicing esse enim consequat tempor do ad. Velit nulla reprehenderit eu eu culpa sint nisi nisi proident in mollit.\r\n",
              "registered": "2017-11-19T09:24:52 +08:00",
              "latitude": -45.343587,
              "longitude": -34.525004,
              "tags": [
                "est",
                "laborum",
                "voluptate",
                "qui",
                "nisi",
                "consectetur",
                "voluptate"
              ],
              "friends": [
                {
                  "id": 0,
                  "name": "Josefa Yang"
                },
                {
                  "id": 1,
                  "name": "Judith Smith"
                },
                {
                  "id": 2,
                  "name": "Bette Sweet"
                }
              ],
              "greeting": "Hello, Camacho Robles! You have 2 unread messages.",
              "favoriteFruit": "apple"
            },
            {
              "_id": "62391b999b8d755f3a526b8d",
              "index": 1,
              "guid": "33e72e80-3cb4-467d-9cc8-be79513b2df7",
              "isActive": true,
              "balance": "$2,472.02",
              "picture": "http://placehold.it/32x32",
              "age": 24,
              "eyeColor": "brown",
              "name": "Alberta Gutierrez",
              "gender": "female",
              "company": "XSPORTS",
              "email": "albertagutierrez@xsports.com",
              "phone": "+1 (800) 400-3952",
              "address": "921 Seaview Court, Watrous, Puerto Rico, 2515",
              "about": "Dolore culpa cillum elit ut consectetur. Duis cupidatat cillum proident excepteur. Culpa magna ut nulla nostrud commodo labore qui dolore incididunt enim nisi officia sint exercitation.\r\n",
              "registered": "2021-11-06T12:27:09 +07:00",
              "latitude": 32.775981,
              "longitude": -137.94526,
              "tags": [
                "voluptate",
                "esse",
                "est",
                "laborum",
                "consequat",
                "Lorem",
                "minim"
              ],
              "friends": [
                {
                  "id": 0,
                  "name": "Shari Wilson"
                },
                {
                  "id": 1,
                  "name": "Britt Cain"
                },
                {
                  "id": 2,
                  "name": "Hunt Walls"
                }
              ],
              "greeting": "Hello, Alberta Gutierrez! You have 10 unread messages.",
              "favoriteFruit": "apple"
            },
            {
              "_id": "62391b99ae4f908e2a307ddb",
              "index": 2,
              "guid": "4404e3f5-5b21-42a1-b56e-954e5d8cbcb6",
              "isActive": false,
              "balance": "$3,272.26",
              "picture": "http://placehold.it/32x32",
              "age": 22,
              "eyeColor": "brown",
              "name": "Whitfield Gay",
              "gender": "male",
              "company": "TURNABOUT",
              "email": "whitfieldgay@turnabout.com",
              "phone": "+1 (981) 594-3973",
              "address": "971 Dupont Street, Hilltop, Mississippi, 2878",
              "about": "Amet commodo proident tempor aliqua et. Lorem ex aliqua proident sint. Consequat id laboris officia ipsum reprehenderit consectetur est ad.\r\n",
              "registered": "2016-08-08T08:14:59 +07:00",
              "latitude": -21.059653,
              "longitude": -114.624733,
              "tags": [
                "reprehenderit",
                "labore",
                "ut",
                "do",
                "Lorem",
                "sint",
                "ut"
              ],
              "friends": [
                {
                  "id": 0,
                  "name": "Lacey Mcclure"
                },
                {
                  "id": 1,
                  "name": "Giles Reynolds"
                },
                {
                  "id": 2,
                  "name": "Chambers Mejia"
                }
              ],
              "greeting": "Hello, Whitfield Gay! You have 5 unread messages.",
              "favoriteFruit": "banana"
            },
            {
              "_id": "62391b9918509c6bdccffaa6",
              "index": 3,
              "guid": "dbae9145-9349-4d74-8b77-ea822e778662",
              "isActive": false,
              "balance": "$1,610.48",
              "picture": "http://placehold.it/32x32",
              "age": 36,
              "eyeColor": "brown",
              "name": "Sue Guerra",
              "gender": "female",
              "company": "SEALOUD",
              "email": "sueguerra@sealoud.com",
              "phone": "+1 (909) 457-2764",
              "address": "407 Maple Avenue, Carlton, District Of Columbia, 1623",
              "about": "Sunt proident duis eu nostrud ullamco elit exercitation. Mollit nulla culpa irure exercitation aliquip reprehenderit incididunt. Amet quis ex mollit mollit ipsum pariatur quis consequat. Adipisicing fugiat proident ad fugiat laborum est enim eu exercitation eiusmod ea mollit consectetur irure. Nulla eiusmod veniam cupidatat reprehenderit tempor aliquip officia. Do nostrud aute magna ipsum Lorem sunt pariatur enim commodo. Non amet mollit officia ullamco non anim exercitation.\r\n",
              "registered": "2016-03-09T12:10:35 +08:00",
              "latitude": 46.415397,
              "longitude": -33.0734,
              "tags": [
                "labore",
                "aute",
                "consectetur",
                "excepteur",
                "quis",
                "Lorem",
                "sunt"
              ],
              "friends": [
                {
                  "id": 0,
                  "name": "Grant Marks"
                },
                {
                  "id": 1,
                  "name": "Osborne Rowe"
                },
                {
                  "id": 2,
                  "name": "Carr Medina"
                }
              ],
              "greeting": "Hello, Sue Guerra! You have 9 unread messages.",
              "favoriteFruit": "banana"
            },
            {
              "_id": "62391b99ab3ac8f109ef0be6",
              "index": 4,
              "guid": "50a944e2-d8b5-4b7e-940c-61c9e774a5e6",
              "isActive": true,
              "balance": "$3,001.24",
              "picture": "http://placehold.it/32x32",
              "age": 31,
              "eyeColor": "brown",
              "name": "Mccormick Davidson",
              "gender": "male",
              "company": "COMTRACT",
              "email": "mccormickdavidson@comtract.com",
              "phone": "+1 (914) 561-2608",
              "address": "546 Murdock Court, Wanship, Rhode Island, 1048",
              "about": "Pariatur cillum aliquip tempor labore consequat Lorem fugiat aliquip in mollit cupidatat. Ea quis esse magna exercitation mollit incididunt sit eu enim. Pariatur quis fugiat ullamco consequat consequat ullamco. Consequat officia irure et magna cillum deserunt consequat officia minim aliquip sint minim ea. Aliqua reprehenderit fugiat officia magna do. In duis excepteur nisi nulla dolor est dolore enim nostrud dolore reprehenderit do. Laboris magna aliqua pariatur elit dolor commodo sit.\r\n",
              "registered": "2021-07-09T08:43:09 +07:00",
              "latitude": -86.298276,
              "longitude": -63.174574,
              "tags": [
                "exercitation",
                "sunt",
                "do",
                "quis",
                "irure",
                "magna",
                "ullamco"
              ],
              "friends": [
                {
                  "id": 0,
                  "name": "Tommie Bullock"
                },
                {
                  "id": 1,
                  "name": "Brennan Mcleod"
                },
                {
                  "id": 2,
                  "name": "Sadie Robbins"
                }
              ],
              "greeting": "Hello, Mccormick Davidson! You have 4 unread messages.",
              "favoriteFruit": "banana"
            }
          ])) }),
    ).route(
        "/server-route-2",
        routing::get(|| async { Json(json! ({"b": 2})) }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
