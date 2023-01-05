import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import {useEffect, useState} from "react";



function NavigationComponent() {

    const logOutHandler = () => {
        localStorage.removeItem("user-token");
        console.log(localStorage.getItem("user-token"))
    }

    const signInNav = <Nav.Link href="/login"><span>Sign In</span></Nav.Link>;
    const logOutNav = <Nav.Link onClick={logOutHandler} href="/login"><span>Log out</span></Nav.Link>;

    const [authNav, setAuthNav] = useState(signInNav);

    const authHandler = () => {
        const token = localStorage.getItem("user-token");
        if (token === undefined || token === null) {
            setAuthNav(signInNav);
        } else {
            setAuthNav(logOutNav);
        }
    }

    useEffect( () => {
       authHandler();
    }, [])


    return (
        <Navbar className="nav-header" collapseOnSelect expand="lg" bg="dark" variant="dark">
            <Container>
                <Navbar.Brand className="logo-text" href="/">Junior Club</Navbar.Brand>
                <Navbar.Toggle aria-controls="responsive-navbar-nav"/>
                <Navbar.Collapse id="responsive-navbar-nav">
                    <Nav className="me-auto my-auto nav-text">
                        <Nav.Link href="/java">Java</Nav.Link>
                        <Nav.Link href="/rust">Rust</Nav.Link>
                    </Nav>
                    <Nav>
                        {authNav}
                    </Nav>
                </Navbar.Collapse>
            </Container>
        </Navbar>
    );
}

export default NavigationComponent;