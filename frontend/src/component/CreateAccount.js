import {useState} from "react";
import axios from "axios";
import {useNavigate} from "react-router-dom";


function CreateAccountComponent() {

    const createUserUrl = 'http://localhost:8000/user/create';

    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const [reEnterPassword, setReEnterPassword] = useState("");
    const [email, setEmail] = useState("");
    const navigate = useNavigate();


    const createUserHandler = () => {
        console.log(name)
        
        if (isLoginValid(name) && isPasswordValid(password) && isEqual(password, reEnterPassword) && isEmailValid(email)) {
            axios({
                method: 'post',
                url: createUserUrl,
                data: {
                    name: name,
                    email: email,
                    password: password
                }
            })
                .then(() => {
                    navigate('/')
                })
                .catch(err => console.log(err))
        }
    }

    return (
        <div className="d-flex justify-content-center">
            <div className="p-5 mt-5">

                <div className="text-color mt-5">
                    Create account
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

                <div className="pt-3">
                    <input className="background-color text-color border-lite-slim"
                           placeholder="Re-enter password"
                           value={reEnterPassword}
                           onChange={e => setReEnterPassword(e.target.value)}
                    />
                </div>

                <div className="pt-3">
                    <input className="background-color text-color border-lite-slim"
                           placeholder="Email"
                           value={email}
                           onChange={e => setEmail(e.target.value)}
                    />
                </div>

                <button
                    className="border-lite background-color text-color mt-3 col-12"
                    onClick={createUserHandler}>

                    <span>Create</span>
                </button>

            </div>
        </div>
    )
}

function isPasswordValid(value) {
    const passwordPattern = /^(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{6,20}$/;

    if(value.match(passwordPattern)) {
        return true;
    } else {
        alert('Пароль должен содержать от 7 до 20 символов и по крайней мере одну цифру, одну заглавную и одну строчную букву')
        return false;
    }
}

function isLoginValid(value) {
    const loginPattern = /^[A-Za-z]\w{4,15}$/;

    if(value.match(loginPattern)) {
        return true;
    } else {
        alert('Логин должен содержать от 5 до 16 символов. Содержит только символы, числовые цифры, подчеркивание и первый символ должны быть буквой')
        return false;
    }
}

function isEqual(val1, val2) {
    if (val1 === val2) {
        return true;
    } else {
        alert('Пароли не совпадают')
        return false;
    }
}

function isEmailValid(val) {
    const emailPattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;

    if (val.match(emailPattern)) {
        return true;
    } else {
        alert('Please enter a valid Email Address')
    }
}




export default CreateAccountComponent;