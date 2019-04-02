let aGroups;
let aStores;

function showShoppingList() {
    deactivateAllTabs();
    hideElements(["c_recipes", "c_ingredients"]);
    document.getElementById("btnShoppingList").className += " active";
    document.getElementById("btnShoppingList").style.width = "40%";
    cleanContent("c_shoppingList");
    showElement(["c_shoppingList", "shoppingList"]);
    return getShoppingList()
        .then(() => {
            return getAllIngredients();
        });
}

function showRecipes() {
    deactivateAllTabs();
    hideElements(["c_shoppingList", "c_ingredients", "shoppingList"]);
    document.getElementById("btnRecipes").className += " active";
    document.getElementById("btnRecipes").style.width = "40%";
    cleanContent("c_recipes");
    showElement(["c_recipes"]);
}

function showIngredients() {
    deactivateAllTabs();
    hideElements(["c_shoppingList", "c_recipes", "shoppingList"]);
    document.getElementById("btnIngredients").className += " active";
    document.getElementById("btnIngredients").style.width = "40%";
    cleanContent("c_ingredients");
    showElement(["c_ingredients"]);
    displayIngredients();
}

function displayIngredients() {
    const oContent = document.getElementById("c_ingredients");
    let oNewContent = [];

    return getAllStores()
        .then(() => {
            return getAllGroups();
        }).then(() => {
            return getQuery("/ingredient")
        })
        .then((ingredients) => {
            const oList = document.createElement("ul");

            ingredients.forEach((ingredient) => {
                const oIngredient = document.createElement("li");

                //name
                const oName = document.createElement("span");
                oName.setAttribute("id", `name_${ingredient.name}`);
                oName.innerText = ingredient.name;
                oIngredient.appendChild(oName);

                //group
                const oGroup = document.createElement("select");
                oGroup.setAttribute("id", `group_${ingredient.name}`);
                addOptions(aGroups, oGroup);
                oGroup.value = aGroups.indexOf(ingredient.group);
                oGroup.addEventListener('change', handleChange, false);
                oIngredient.appendChild(oGroup);

                //store
                const oStore = document.createElement("select");
                oStore.setAttribute("id", `store_${ingredient.name}`);
                addOptions(aStores, oStore);
                oStore.value = aStores.indexOf(ingredient.store);
                oStore.addEventListener('change', handleChange, false);
                oIngredient.appendChild(oStore);

                oList.appendChild(oIngredient);
            });

            oNewContent.push(oList);
        })
        .catch((err) => {
            oNewContent = document.createElement('span');
            oNewContent.innerText = err;
        })
        .finally(() => {
            if (Array.isArray(oNewContent)) {
                oNewContent.forEach((newContent) => {
                    oContent.appendChild(newContent);
                });
            }
            else {
                oContent.appendChild(oNewContent);
            }
        });
}

function handleChange(e) {
    const sIngredient = e.target.id.substring(e.target.id.indexOf('_') + 1);
    const iGroup = document.getElementById(`group_${sIngredient}`).selectedIndex;
    const iStore = document.getElementById(`store_${sIngredient}`).selectedIndex;
    putData("ingredient", [sIngredient, iGroup, iStore]);
}

function addOptions(aArr, oSelect) {
    aArr.forEach((sOption, i) => {
        const oOption = document.createElement('option');
        oOption.value = i;
        oOption.innerHTML = sOption;
        oSelect.appendChild(oOption);
    });
}

function deactivateAllTabs() {
    const aTabs = document.getElementsByClassName("tablinks");
    for (let oTab of aTabs) {
        oTab.className = oTab.className.replace(" active", "");
        oTab.style.width = "30%";
    };
}

function hideElements(aId) {
    aId.forEach((sId) => {
        document.getElementById(sId).style.display = "none";
    });
}

function showElement(aId) {
    aId.forEach((sId) => {
        document.getElementById(sId).style.display = "block";
    });
}

function cleanContent(sId) {
    if (!isOffline) {
        const oContent = document.getElementById(sId);
        while (oContent.firstChild) {
            oContent.removeChild(oContent.firstChild);
        }
    }
}

function cleanOptions() {
    const oIngredients = document.getElementById("dl_ingredients");
    while (oIngredients.children.length > 0) {
        oIngredients.children[0].remove();
    }
}

function isOffline() {
    return document.getElementById("cbOffline").checked;
}

function ajax(sMethod, sUrl) {
    if (isOffline()) {
        return Promise.resolve([]);
    }

    return new Promise((resolve, reject) => {
        const xmlHttp = new XMLHttpRequest();
        xmlHttp.onreadystatechange = () => {
            if (xmlHttp.readyState == 4) {
                if (xmlHttp.status == 200) {
                    resolve(JSON.parse(xmlHttp.response));
                }
                else {
                    reject(xmlHttp.responseText);
                }
            }
        }
        xmlHttp.open(sMethod, sUrl, true);
        xmlHttp.setRequestHeader("Content-Type", "application/json");
        xmlHttp.send(null);
    });
}

function deleteData(sUrl, sData) {
    return ajax("DELETE", `${sUrl}/${sData}`);
}

function putData(sUrl, aData) {
    return ajax("PUT", `${sUrl}/${aData.join('/')}`);
}

function getQuery(sUrl) {
    return ajax("GET", sUrl);
}

function getAllStores() {
    if (!aStores) {
        return getQuery("/store")
            .then((data) => {
                aStores = data.stores;
                return aStores;
            });
    }
    return Promise.resolve(aStores);
}

function getAllGroups() {
    if (!aGroups) {
        return getQuery("/group")
            .then((data) => {
                aGroups = data.groups;
                return aGroups;
            });
    }
    return Promise.resolve(aGroups);
}

function getAllIngredients() {
    cleanOptions();
    return getQuery("/ingredient")
        .then((ingredients) => {
            const oIngredients = document.getElementById("dl_ingredients");
            ingredients.forEach((i) => {
                const oOption = document.createElement('option');
                oOption.value = i.name;
                oIngredients.appendChild(oOption);
            });
        });
}

function displayShoppingList(shoppingList) {
    const oContent = document.getElementById("c_shoppingList");
    let oNewContent = [];
    return Promise.resolve().then(() => {

        Object.keys(shoppingList).forEach((store) => {
            if (store !== "Any") {
                const oStoreHeader = document.createElement('h1');
                oStoreHeader.innerText = store;
                oNewContent.push(oStoreHeader);
            }
            Object.keys(shoppingList[store]).forEach((category) => {
                const oList = document.createElement('ul');
                oNewContent.push(oList);

                shoppingList[store][category].forEach((item) => {
                    const oItem = document.createElement('li');
                    oItem.setAttribute("id", `li_${item.name}`);
                    oItem.setAttribute("draggable", true);

                    if (item.amount) {
                        oItem.appendChild(document.createTextNode(`${item.name}: ${item.amount}`));
                    } else {
                        oItem.appendChild(document.createTextNode(item.name));
                    }
                    oItem.addEventListener('dragstart', handleDragStart, false);
                    oItem.addEventListener('dragend', handleDragEnd, false);

                    oItem.addEventListener('touchstart', handleTouchStart, { passive: true });
                    oItem.addEventListener('touchmove', handleTouchMove, { passive: true });
                    oItem.addEventListener('touchend', handleTouchEnd, { passive: true });

                    oList.appendChild(oItem);

                    const oButton = document.createElement('button');
                    oButton.setAttribute("id", item.name);
                    oButton.className = "delete";
                    oButton.innerText = 'X';

                    oButton.addEventListener('click', () => { removeIngredient(item.name) }, false);
                    oItem.appendChild(oButton);
                });
            });

            oNewContent.push(document.createElement("hr"));
        });
    })
        .catch((err) => {
            oNewContent = document.createElement('span');
            oNewContent.innerText = err;
        }).finally(() => {
            if (Array.isArray(oNewContent)) {
                oNewContent.forEach((newContent) => {
                    oContent.appendChild(newContent);
                });
            }
            else {
                oContent.appendChild(oNewContent);
            }
        });
}

function getShoppingList() {
    cleanContent("c_shoppingList");
    return getQuery("/shopping_list")
        .then((shoppingList) => {
            return displayShoppingList(shoppingList);
        });
}

function addIngredient() {
    const oIngredient = document.getElementById("newIngredient");
    const sInput = oIngredient.value;

    cleanContent("c_shoppingList");
    return putData("ingredient", [sInput])
        .then((shoppingList) => {
            displayShoppingList(shoppingList);
        })
        .then(() => {
            return getAllIngredients();
        });
}

function removeIngredient(sIngredient) {
    if(isOffline()) {
        document.getElementById(`li_${sIngredient}`).outerHTML = "";
    }

    const iPosition = document.documentElement.scrollTop || document.body.scrollTop;
    cleanContent("c_shoppingList");
    return deleteData("ingredient", sIngredient)
        .then((shoppingList) => {
            displayShoppingList(shoppingList);
        })
        .then(() => {
            return getAllIngredients();
        }).finally(() => {
            window.scrollTo(0, iPosition);
        });
}

const DELTA = 100;
let iX = 0;
function handleDragStart(e) {
    iX = e.clientX;
    e.target.style.opacity = "0.4";
}

function handleDragEnd(e) {
    if (Math.abs(iX - e.clientX) > DELTA) {
        const sId = e.target.id;
        if (sId.startsWith("li_")) {
            removeIngredient(sId.substr(3));
        }
    }
    else {
        e.target.style.opacity = "1";
    }
    iX = 0;
}

function handleTouchStart(e) {
    iX = e.targetTouches[0].pageX;
    e.target.style.opacity = "0.4";
}

function handleTouchMove(e) {
    const touchLocation = e.targetTouches[0];
    e.target.style.left = touchLocation.pageX + 'px';
    e.target.style.top = touchLocation.pageY + 'px';
}

function handleTouchEnd(e) {
    if (Math.abs(iX - e.changedTouches[0].screenX) > DELTA) {
        const sId = e.target.id;
        if (sId.startsWith("li_")) {
            removeIngredient(sId.substr(3));
        }
    }
    else {
        e.target.style.opacity = "1";
    }
    iX = 0;
}