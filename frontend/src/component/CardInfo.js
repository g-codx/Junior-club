import {Card} from "react-bootstrap";
import axios from "axios";
import {useEffect, useState} from "react";


const token = localStorage.getItem("user-token");

function CardInfoComponent(section) {

    let [resourceLinks, setResourceLinks] = useState([]);
    let [bookLinks, setBookLinks] = useState([]);
    const [isAdmin, setIsAdmin] = useState(false);
    const [fetching, setFetching] = useState(true);

    useEffect(() => {
        if (fetching) {
            getLinks(section.section)
                .then(response => {
                    const resourceList = response.data.links.filter(l => l.link_type === 'resource');
                    const booksList = response.data.links.filter(l => l.link_type === 'book');
                    setResourceLinks([...resourceLinks, ...resourceList]);
                    setBookLinks([...bookLinks, ...booksList])
                    setIsAdmin(response.data.is_admin);
                })
                .finally(() => setFetching(false))
        }
    });

    const editModeHandler = (linkId) => {
        document.getElementById(linkId).classList.toggle("d-none");
    }

    const addModeHandler = (title) => {
        document.getElementById(title).classList.toggle("d-none");
    }

    const addNewLinkHandler = (linkType, section) => {
        const title = document.getElementById("new-" + linkType + "-title").value;
        const link = document.getElementById("new-" + linkType + "-link").value;

        axios({
            method: 'post',
            url: 'http://localhost:8000/link/create',
            data: {
                title: title,
                link: link,
                link_type: linkType,
                section_type: section
            },
            headers: {
                'user-token': token
            }
        }).then(r => {
            updateContainer();
            console.log(r);
        })
    }

    const deleteHandler = (linkId) => {
        axios({
            method: 'delete',
            url: 'http://localhost:8000/link/delete/' + linkId,
            headers: {
                'user-token': token
            }
        }).then(r => {
            updateContainer();
            console.log(r);
        })
    }

    const editHandler = (linkType, linkId) => {
        const title = document.getElementById("changed-" + linkId + "-title").value;
        const link = document.getElementById("changed-" + linkId + "-link").value;

        axios({
            method: 'put',
            url: 'http://localhost:8000/link/edit',
            data: {
                id: linkId,
                title: title,
                link: link
            },
            headers: {
                'user-token': token
            }
        }).then(r => {
            updateContainer();
            console.log(r);
        })
    }

    const updateContainer = () => {
        setResourceLinks([]);
        setBookLinks([]);
        setFetching(true);
    }

    const addModeElement = (title) => isAdmin ?
        <span onClick={() => addModeHandler(title)} className="add-icon material-icons">add</span> :
        <></>

    const editModeElement = (linkId) => isAdmin ?
        <span className="edit-links material-icons edit-icon"
              onClick={() => editModeHandler(linkId)}>edit_document</span> :
        <></>

    const content = (linkType, links) => {
        let title = linkType === 'resource' ? 'Полезные ресурсы' : 'Книги';

        return (
            <div className="p-4">

                <div>
                    <Card.Header>{title}</Card.Header>
                    {addModeElement(title)}
                </div>

                <Card.Body>
                    <ul id={"ul-" + linkType}>
                        {links.map(link =>
                            <li key={link.id}>

                                <div className="p-2">
                                    <a className="resource-link" id={"link-id" + link.id} href={link.link} target="example" rel="noopener">
                                        <div id={"title-id" + link.id}>
                                            {link.title}
                                        </div>
                                    </a>

                                    {editModeElement(link.id)}
                                </div>

                                <div id={link.id} className="d-none d-flex justify-content-around flex-column mb-4">
                                    <input id={"changed-" + link.id + "-title"} defaultValue={link.link}/>
                                    <input id={"changed-" + link.id + "-link"} defaultValue={link.title}/>
                                    <button onClick={() => editHandler(linkType, link.id)}>Save</button>
                                    <button onClick={() => deleteHandler(link.id)}>Delete</button>
                                </div>

                            </li>
                        )}
                    </ul>

                    <div id={title} className="d-none d-flex justify-content-around flex-column mb-4">
                        <input id={"new-" + linkType + "-title"} placeholder="Title"/>
                        <input id={"new-" + linkType + "-link"} placeholder="Link"/>
                        <button onClick={() => addNewLinkHandler(linkType, section.section)}>Add</button>
                    </div>
                </Card.Body>
            </div>
        )
    };


    return (
//justify-content-center
        <Card className="card-info background-color text-color d-flex align-items-center p-5">
            {content('resource', resourceLinks)}
            {content('book', bookLinks)}
        </Card>
    )
}


function getLinks(section) {
    return axios({
        method: 'get',
        url: 'http://localhost:8000/link/get',
        params: {
            section: section,
        },
        headers: {
            'user-token': token
        }
    })
}

export default CardInfoComponent;
