import {useEffect, useState} from "react";
import axios from "axios";
import {useNavigate} from "react-router-dom";

function PostSendFormComponent() {
    const navigate = useNavigate();
    const [searchTag, setSearchTag] = useState("All")
    const [title, setTitle] = useState("");
    const [body, setBody] = useState("");

    useEffect( () => {
        const token = localStorage.getItem("user-token");
        if (token === undefined || token === null) {
            navigate('/login')
        }
        console.log("check")
    }, [])

    const handleSubmit = () => {
        const token = localStorage.getItem("user-token");
        console.log(token);
        axios({
            method: 'post',
            url: 'http://localhost:8000/post/create',
            withCredentials: false,
            headers: {
                'user-token': token
            },
            data: {
                title: title,
                body: body,
                search_tag: searchTag,
            }
        }).then(() => navigate('/'))
    }

    const bodyTextHandler = (e) => {
        const text = e.target.value;
        setBody(text);
    }

    return (
        <div className="col d-flex align-items-center justify-content-center">
            <div className="col-8 p-5">
                <form onSubmit={handleSubmit} id="form">
                    <fieldset>
                        <div className="mb-3">
                            <select
                                className="col-3 border-lite background-color text-color"
                                id="tag"
                                aria-label="tag"
                                value={searchTag}
                                onChange={(e) => setSearchTag(e.target.value)}
                            >
                                <option>All</option>
                                <option>Java</option>
                                <option>Rust</option>
                            </select>
                        </div>

                        <div className="mb-3">
                        <textarea
                            className="col-12 border-lite background-color text-color vh-5 textarea_send_form"
                            id="title"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            placeholder="Title"
                            maxLength="120"
                        >
                        </textarea>
                        </div>

                        <div className="mb-3">
                        <textarea
                            className="col-12 border-lite background-color text-color _textarea vh-70 textarea_send_form"
                            id="text"
                            value={body}
                            onChange={bodyTextHandler}
                            placeholder="Text"
                            cols="5"
                            rows="5"
                            wrap="hard"
                        >
                        </textarea>
                        </div>
                    </fieldset>

                </form>

                <div className="mb-3">
                    <button onClick={handleSubmit} className="col-3 border-lite background-color text-color">Send</button>
                </div>
            </div>
        </div>
    )
}

export default PostSendFormComponent;