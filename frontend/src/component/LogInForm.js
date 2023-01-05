import {useState} from "react";
import axios from "axios";
import {useNavigate} from "react-router-dom";


function LogInComponent() {

    const [name, setName] = useState("");
    const [password, setPassword] = useState("");

    const authUrl = 'http://localhost:8000/auth/login';
    const navigate = useNavigate();


    const authHandler = () => {
        axios({
            method: 'post',
            url: authUrl,

            data: {
                username: name,
                password: password
            }
        })
            .then(response => {
                const token = response.headers.get("authorization");
                localStorage.setItem("user-token", token);
                console.log(localStorage.getItem("user-token"));
                navigate('/');
            })
            .catch(err => {
                alert("Неверный логи или пароль. Попробуйте снова.")
                console.log(err);
            })
    }

    return (
        <div className="d-flex justify-content-center">
            <div className="p-5 mt-5">

                <div className="text-color mt-5 fw-bold">
                    Sign In
                </div>

                <div className="pt-3">
                    <input
                        className="background-color text-color border-lite-slim"
                        placeholder="Your name"
                        value={name}
                        onChange={e => setName(e.target.value)}
                    />
                </div>

                <div className="pt-3">
                    <input
                        className="background-color text-color border-lite-slim"
                        placeholder="Password"
                        value={password}
                        onChange={e => setPassword(e.target.value)}
                    />
                </div>


                <button
                    className="border-lite background-color text-color mt-3 col-12"
                    onClick={authHandler}>

                    <span>Sign In</span>
                </button>

                <div className="mt-3">
                    <a className="link" href="/registration">Create new</a>
                </div>

            </div>
        </div>
    )
}

export default LogInComponent;