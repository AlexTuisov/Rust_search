(define (problem zenotravel-problem-12)
  (:domain zenotravel)
  (:objects
    plane1 - aircraft
    plane2 - aircraft
    plane3 - aircraft
    plane4 - aircraft
    plane5 - aircraft
    person1 - person
    person2 - person
    person3 - person
    person4 - person
    person5 - person
    person6 - person
    person7 - person
    person8 - person
    person9 - person
    person10 - person
    person11 - person
    person12 - person
    person13 - person
    person14 - person
    person15 - person
    city0 - city
    city1 - city
    city2 - city
    city3 - city
    city4 - city
    city5 - city
    city6 - city
    city7 - city
    city8 - city
    city9 - city
    city10 - city
    city11 - city)
  (:init
    ;; Plane 1
    (located plane1 city0)
    (= (capacity plane1) 3000)
    (= (fuel plane1) 819)
    (= (slow-burn plane1) 1)
    (= (fast-burn plane1) 2)
    (= (onboard plane1) 0)
    (= (zoom-limit plane1) 2)

    
    ;; Plane 2
    (located plane2 city3)
    (= (capacity plane2) 5487)
    (= (fuel plane2) 1061)
    (= (slow-burn plane2) 2)
    (= (fast-burn plane2) 5)
    (= (onboard plane2) 0)
    (= (zoom-limit plane2) 3)
 
    
    ;; Plane 3
    (located plane3 city2)
    (= (capacity plane3) 11420)
    (= (fuel plane3) 3655)
    (= (slow-burn plane3) 5)
    (= (fast-burn plane3) 12)
    (= (onboard plane3) 0)
    (= (zoom-limit plane3) 4)

    
    ;; Plane 4
    (located plane4 city9)
    (= (capacity plane4) 2641)
    (= (fuel plane4) 141)
    (= (slow-burn plane4) 1)
    (= (fast-burn plane4) 2)
    (= (onboard plane4) 0)
    (= (zoom-limit plane4) 1)

    
    ;; Plane 5
    (located plane5 city5)
    (= (capacity plane5) 9016)
    (= (fuel plane5) 50)
    (= (slow-burn plane5) 4)
    (= (fast-burn plane5) 9)
    (= (onboard plane5) 0)
    (= (zoom-limit plane5) 4)

    
    ;; Persons
    (located person1 city8)
    (located person2 city10)
    (located person3 city7)
    (located person4 city5)
    (located person5 city1)
    (located person6 city10)
    (located person7 city11)
    (located person8 city8)
    (located person9 city9)
    (located person10 city11)
    (located person11 city4)
    (located person12 city5)
    (located person13 city8)
    (located person14 city4)
    (located person15 city1)
    
    ;; Distances between cities
    (= (distance city0 city0) 0)
    (= (distance city0 city1) 804)
    (= (distance city0 city2) 709)
    (= (distance city0 city3) 610)
    (= (distance city0 city4) 745)
    (= (distance city0 city5) 872)
    (= (distance city0 city6) 881)
    (= (distance city0 city7) 608)
    (= (distance city0 city8) 948)
    (= (distance city0 city9) 522)
    (= (distance city0 city10) 632)
    (= (distance city0 city11) 578)
    (= (distance city1 city0) 804)
    (= (distance city1 city1) 0)
    (= (distance city1 city2) 936)
    (= (distance city1 city3) 654)
    (= (distance city1 city4) 605)
    (= (distance city1 city5) 771)
    (= (distance city1 city6) 585)
    (= (distance city1 city7) 966)
    (= (distance city1 city8) 896)
    (= (distance city1 city9) 580)
    (= (distance city1 city10) 881)
    (= (distance city1 city11) 675)
    (= (distance city2 city0) 709)
    (= (distance city2 city1) 936)
    (= (distance city2 city2) 0)
    (= (distance city2 city3) 511)
    (= (distance city2 city4) 640)
    (= (distance city2 city5) 590)
    (= (distance city2 city6) 761)
    (= (distance city2 city7) 655)
    (= (distance city2 city8) 846)
    (= (distance city2 city9) 968)
    (= (distance city2 city10) 612)
    (= (distance city2 city11) 727)
    (= (distance city3 city0) 610)
    (= (distance city3 city1) 654)
    (= (distance city3 city2) 511)
    (= (distance city3 city3) 0)
    (= (distance city3 city4) 832)
    (= (distance city3 city5) 916)
    (= (distance city3 city6) 936)
    (= (distance city3 city7) 942)
    (= (distance city3 city8) 662)
    (= (distance city3 city9) 808)
    (= (distance city3 city10) 823)
    (= (distance city3 city11) 770)
    (= (distance city4 city0) 745)
    (= (distance city4 city1) 605)
    (= (distance city4 city2) 640)
    (= (distance city4 city3) 832)
    (= (distance city4 city4) 0)
    (= (distance city4 city5) 757)
    (= (distance city4 city6) 846)
    (= (distance city4 city7) 903)
    (= (distance city4 city8) 835)
    (= (distance city4 city9) 782)
    (= (distance city4 city10) 557)
    (= (distance city4 city11) 941)
    (= (distance city5 city0) 872)
    (= (distance city5 city1) 771)
    (= (distance city5 city2) 590)
    (= (distance city5 city3) 916)
    (= (distance city5 city4) 757)
    (= (distance city5 city5) 0)
    (= (distance city5 city6) 554)
    (= (distance city5 city7) 642)
    (= (distance city5 city8) 907)
    (= (distance city5 city9) 950)
    (= (distance city5 city10) 723)
    (= (distance city5 city11) 788)
    (= (distance city6 city0) 881)
    (= (distance city6 city1) 585)
    (= (distance city6 city2) 761)
    (= (distance city6 city3) 936)
    (= (distance city6 city4) 846)
    (= (distance city6 city5) 554)
    (= (distance city6 city6) 0)
    (= (distance city6 city7) 625)
    (= (distance city6 city8) 734)
    (= (distance city6 city9) 929)
    (= (distance city6 city10) 715)
    (= (distance city6 city11) 995)
    (= (distance city7 city0) 608)
    (= (distance city7 city1) 966)
    (= (distance city7 city2) 655)
    (= (distance city7 city3) 942)
    (= (distance city7 city4) 903)
    (= (distance city7 city5) 642)
    (= (distance city7 city6) 625)
    (= (distance city7 city7) 0)
    (= (distance city7 city8) 585)
    (= (distance city7 city9) 562)
    (= (distance city7 city10) 964)
    (= (distance city7 city11) 697)
    (= (distance city8 city0) 948)
    (= (distance city8 city1) 896)
    (= (distance city8 city2) 846)
    (= (distance city8 city3) 662)
    (= (distance city8 city4) 835)
    (= (distance city8 city5) 907)
    (= (distance city8 city6) 734)
    (= (distance city8 city7) 585)
    (= (distance city8 city8) 0)
    (= (distance city8 city9) 789)
    (= (distance city8 city10) 797)
    (= (distance city8 city11) 614)
    (= (distance city9 city0) 522)
    (= (distance city9 city1) 580)
    (= (distance city9 city2) 968)
    (= (distance city9 city3) 808)
    (= (distance city9 city4) 782)
    (= (distance city9 city5) 950)
    (= (distance city9 city6) 929)
    (= (distance city9 city7) 562)
    (= (distance city9 city8) 789)
    (= (distance city9 city9) 0)
    (= (distance city9 city10) 726)
    (= (distance city9 city11) 739)
    (= (distance city10 city0) 632)
    (= (distance city10 city1) 881)
    (= (distance city10 city2) 612)
    (= (distance city10 city3) 823)
    (= (distance city10 city4) 557)
    (= (distance city10 city5) 723)
    (= (distance city10 city6) 715)
    (= (distance city10 city7) 964)
    (= (distance city10 city8) 797)
    (= (distance city10 city9) 726)
    (= (distance city10 city10) 0)
    (= (distance city10 city11) 776)
    (= (distance city11 city0) 578)
    (= (distance city11 city1) 675)
    (= (distance city11 city2) 727)
    (= (distance city11 city3) 770)
    (= (distance city11 city4) 941)
    (= (distance city11 city5) 788)
    (= (distance city11 city6) 995)
    (= (distance city11 city7) 697)
    (= (distance city11 city8) 614)
    (= (distance city11 city9) 739)
    (= (distance city11 city10) 776)
    (= (distance city11 city11) 0)
    (= (total-fuel-used) 0)

  )
  (:goal (and
    (located person1 city1)
    (located person2 city4)
    (located person3 city7)
    (located person4 city6)
    (located person5 city8)
    (located person6 city11)
    (located person7 city2)
    (located person8 city11)
    (located person10 city9)
    (located person11 city6)
    (located person12 city4)
    (located person13 city11)
    (located person14 city4)
    (located person15 city6)))
  (:metric minimize (total-fuel-used))
)
