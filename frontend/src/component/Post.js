import {Spinner} from "react-bootstrap";
import {useEffect, useRef, useState} from "react";
import axios from "axios";

import {useNavigate} from "react-router-dom";

const token = localStorage.getItem("user-token");
const location = window.location.toString()
const id = location.split('/').slice(-1)[0];
const get_url = 'http://localhost:8000/post/get/' + id;
const delete_url = 'http://localhost:8000/post/delete/' + id;
const publish_url = 'http://localhost:8000/post/publish';
const put_url = 'http://localhost:8000/post/edit';
const parsed_id = parseInt(id, 10);

function PostComponent() {

    const readOnlyStyle = {
        display: "block",
        overflow: "hidden",
        resize: "none",
        width: "100%",
        border: "none"
    };

    const editStyle = {
        display: "block",
        overflow: "hidden",
        resize: "none",
        width: "100%",
        border: "0.1rem solid #37968d",
        borderRadius: "0.5rem"
    };

    const textRef = useRef(null);
    const titleRef = useRef(null);

    const navigate = useNavigate();

    const [fetching, setFetching] = useState(true);
    const [isLoaded, setIsLoaded] = useState(false);
    const [statusButton, setStatusButton] = useState(<></>);
    const [currentText, setCurrentText] = useState("");
    const [currentTitle, setCurrentTitle] = useState("");
    const [currentStyle, setCurrentStyle] = useState(readOnlyStyle);
    const [isReadOnly, setIsReadOnly] = useState(true);
    const [showDeleteButton, setShowDeleteButton] = useState(false);
    const [showCancelButton, setShowCancelButton] = useState(false);
    const [isUserCreated, setIsUserCreated] = useState(false);

    useEffect(() => {
        if (fetching) {
             getPost()
                .then(response => {
                    const status = response.data.published;
                    const body = response.data.body;
                    const title = response.data.title;
                    const isUserCreated = response.data.is_user_created;

                    setCurrentText(body);
                    setCurrentTitle(title);
                    setStatusButton(() => buttonHandler(status));
                    setIsUserCreated(isUserCreated);
                    console.log(isUserCreated)

                })
                 .finally(() => {
                     setFetching(false)
                     setIsLoaded(true)
                 })
                 .catch(err => {console.log(err)})
        }
    });

    const deleteHandler = () => {
        axios({
            method: 'delete',
            url: delete_url,
            headers: {
                'user-token': token
            }
        }).then(() => navigate('/'))
    }

    const publishHandler = () => {
       axios({
           method: 'put',
           url: publish_url,
           data: {
               id: parsed_id,
               publish: true
           },
           headers: {
               'user-token': token
           }
       })
           .then(response => console.log(response))
           .finally(() => buttonHandler(true))
    }

    const hideHandler = () => {
        axios({
            method: 'put',
            url: publish_url,
            data: {
                id: parsed_id,
                publish: false
            },
            headers: {
                'user-token': token
            }
        })
            .then(response => console.log(response))
            .finally(() => buttonHandler(false))
    }

    const hideButton = <button id="btn-hide" className="button" onClick={hideHandler}>Hide</button>;
    const publishButton = <button id="btn-show" className="button" onClick={publishHandler}>Publish</button>;

    const buttonHandler = (publish) => {
        if (publish) {
            setStatusButton(hideButton)
        } else {
            setStatusButton(publishButton)
        }
    }

    useEffect(() => {
        if (textRef.current !== null) {
            const scrollHeight = textRef.current.scrollHeight;
            textRef.current.style.height = scrollHeight + "px";
        }
    }, [currentText]);

    useEffect(() => {
        if (titleRef.current !== null) {
            const scrollHeight = titleRef.current.scrollHeight;
            titleRef.current.style.height = scrollHeight + "px";
        }
    }, [currentTitle]);

    const editHandler = () => {
        setSwitchModeButton(pushButton)
        setIsReadOnly(false)
        setCurrentStyle(editStyle)
        setShowDeleteButton(true)
        setShowCancelButton(true)
    }

    const pushHandler = () => {
        setSwitchModeButton(editButton)
        setIsReadOnly(true)
        setCurrentStyle(readOnlyStyle)
        setShowDeleteButton(false)

        const title = titleRef.current.value;
        const text = textRef.current.value;

        axios({
            method: 'put',
            url: put_url,
            data: {
                id: parsed_id,
                title: title,
                body: text
            },
            headers: {
                'user-token': token
            }
        })
            .then(() => {window.location.reload()})
            .catch(err => console.log(err))
    }

    const cancelHandler = () => {
        setSwitchModeButton(editButton)
        setIsReadOnly(true)
        setCurrentStyle(readOnlyStyle)
        setShowDeleteButton(false)
        setShowCancelButton(false)
    }

    const editButton = <button className="button" onClick={editHandler}>Edit</button>
    const pushButton = <button className="button" onClick={pushHandler}>Push</button>

    const [switchModeButton, setSwitchModeButton] = useState(editButton);

    const buttonsBlock = isUserCreated ?
        <div id="button-container" className="col p-3">
            {statusButton}
            {switchModeButton}
            {showDeleteButton ? <button className="button" onClick={deleteHandler}>Delete</button> : null}
            {showCancelButton ? <button className="button" onClick={cancelHandler}>Cancel</button> : null}
        </div> : <></>

    if (isLoaded) {
        return (
            <div className="col">
                <div className="pre-wrap-container col p-5">
                    <div className="col-8">
                        <textarea
                            id="post-area-title"
                            className="textarea-common mb-3 fw-bold"
                            ref={titleRef}
                            style={currentStyle}
                            value={currentTitle}
                            readOnly={isReadOnly}
                            onChange={e => setCurrentTitle(e.target.value)}
                        >
                        {currentTitle}
                        </textarea>
                        <textarea
                            id="post-area-text"
                            className="textarea-common"
                            ref={textRef}
                            style={currentStyle}
                            value={currentText}
                            readOnly={isReadOnly}
                            onChange={e => setCurrentText(e.target.value)}
                        >
                        {currentText}
                        </textarea>
                    </div>
                    {buttonsBlock}
                </div>
            </div>
        )
    } else {
        return (
            <Spinner className="green spinner" animation="border" role="status">
                <span className="visually-hidden">Loading...</span>
            </Spinner>
        )
    }
}

function getPost() {
    return  axios({
        method: 'get',
        url: get_url,
        params: {
            id: id
        },
        headers: {
            'user-token': token
        }
    })
}

export default PostComponent;