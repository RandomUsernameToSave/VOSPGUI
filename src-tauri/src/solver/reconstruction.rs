

pub fn CWENO(th:f64,element_ind_minus_one:f64,element_ind:f64,element_ind_plus_one:f64,element_ind_plus_two:f64,epsi:f64,dx:f64) -> f64{

    let dxi = 1./dx;

    let mut betal =  (element_ind - element_ind_minus_one).powi(2);
    let mut betar= (element_ind_plus_one - element_ind).powi(2);
    let mut betac = ((element_ind_plus_one - 2. * element_ind + element_ind_minus_one).powi(2)) * 13./3. 
            + 0.25 * (( element_ind_plus_one - element_ind_minus_one).powi(2));

    let mut al = 0.25 / ((epsi + betal).powi(2));
    let mut ar = 0.25 / ((epsi + betar).powi(2));
    let mut ac = 0.5 / ((epsi + betac).powi(2));

    betar = ar/ (ar+ac+al);
    betal = al/ (ar+ac+al);
    betac = ac/ (ar+ac+al);

    let r0 = element_ind - 1./12. * betac * (element_ind_plus_one - 2.* element_ind + element_ind_minus_one);
    let r1 =    betal * dxi * (element_ind - element_ind_minus_one)  + 
            betar * dxi * (element_ind_plus_one - element_ind) + 
            betac * 0.5*dxi * (element_ind_plus_one - element_ind_minus_one);
    let r2 = 2.* betac * dxi*dxi*((element_ind_plus_one - 2.* element_ind + element_ind_minus_one));
    

    // SECOND PASS
    // 
    // SECOND PASS 
    

    betal =  (element_ind_plus_one - element_ind).powi(2);
    betar= (element_ind_plus_two - element_ind_plus_one).powi(2);
    betac = 13./3. * ((element_ind_plus_two - 2. * element_ind_plus_one + element_ind).powi(2)) + 0.25 * (( element_ind_plus_two - element_ind).powi(2));

    al = 0.25 / ((epsi + betal).powi(2));
    ar = 0.25 / ((epsi + betar).powi(2));
    ac = 0.5 / ((epsi + betac).powi(2));

    betar = ar/ (ar+ac+al);
    betal = al/ (ar+ac+al);
    betac = ac/ (ar+ac+al);

    let r0p = element_ind_plus_one - 1./12. * betac * (element_ind_plus_two - 2.* element_ind_plus_one + element_ind);
    let r1p =    betal * dxi * (element_ind_plus_one - element_ind)  + 
            betar * dxi * (element_ind_plus_two - element_ind_plus_one) + 
            betac * 0.5*dxi * (element_ind_plus_two - element_ind);
    let r2p = 2.* betac * dxi*dxi*((element_ind_plus_two - 2.* element_ind_plus_one + element_ind));

    // LAST RECONSTRUCTION 
    //
    // LAST RECONSTRUCTION 

    let t0 =(1.-th)*r0 + th*r0p;
    let t1 = dx* th*(1.-th) * (0.5*r1 - 0.5 * r1p);
    let q = 3.*th - 6.*th*th + 4. * th*th*th;
    let t2 = dx*dx /24.* ( (1.-q)*r2 +q*r2p);
    
    t0 + t1 + t2
}