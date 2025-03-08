; Domain Overview and Improvements
; This domain models an industrial supply chain for sugar production, where raw cane is processed into refined sugar.
; It includes various stages such as production at mills, resource acquisition from farms or other mills, and
; transportation using trucks and cranes. The domain employs numeric functions and logical predicates to manage
; resource flows, operational states, and associated costs across the supply chain.
;
; I'm fixing the current domain based on the reference paper below and making changes to improve its optimality
; and clarity.
;
; Reference Paper:
; Coles, Amanda, M. Fox, and D. Long.
; "A hybrid LP-RPG heuristic for modelling numeric resource flows in planning."
; Journal of Artificial Intelligence Research 46 (2013): 343-412.
;
; Author of the New Domain:
; DavidIzhaki
; Email: davidizhakinew@gmail.com

(define (domain supply-chain)
	;(:requirements :typing :fluents :equality)
	(:types 
        ; brand: different types or varieties of sugar.
        ; raw-cane: the unrefined material used to produce sugar.
        ; mill: production facility that processes raw cane into sugar.
        ; depot: storage or distribution facility.
        ; farm-field: field on a farm for harvesting raw cane.
        ; truck: vehicle used for transporting sugar.
        ; crane: mechanical loader used for handling sugar.

		 sugar location loader - object
		 brand raw-cane        - sugar
		 mill depot farm-field - location
		 truck crane	       - loader	
	)
    
	(:predicates
        ; Indicates that mill ?m is available for production
		(available ?m - mill)

        ; Specifies that mill ?m is capable of producing sugar of brand ?b.
		(can-produce ?m - mill ?b - brand)

        ; Denotes the current production process at mill ?m producing sugar of brand ?b.
		(current-brand-process ?m - mill ?b - brand)

        ; Allows the change of production process from brand ?b1 to brand ?b2.
		(change-brand-process ?b1 ?b2 - brand)

        ; Represents that mill ?m has placed an order for raw cane ?r.
		(place-order ?r - raw-cane ?m - mill)

        ; Indicates that the loader (truck, crane, etc.) ?d is at location ?l.
		(at-location ?d - loader  ?l - location)

        ; Specifies that location ?l1 is connected to location ?l2, enabling movement between them.
		(connected ?l1 ?l2 - location)	
	)

    	(:functions

        ; Represents the cumulative cost incurred by all mill operations across the supply chain.
        (mills-total-cost)
        
        ; Represents the cost associated with running the current production process at a specific mill ?m.
        (process-cost ?m - mill)

        ; Represents the maximum amount of sugar that mill ?m can produce in a single production batch.
        (max-produce ?m - mill)
	
        ; Represents the amount of cane that can be harvested per harvest action from a specific farm-field.
        (cane-yield ?f - farm-field)
        
        ; Indicates the total amount of cane currently available on a specific farm-field.
        (total-canes ?f - farm-field)

        ; Represents the maximum amount of sugar that the crane can handle in a single operation.
        (capacity ?c - crane)
  
        ; Indicates the remaining capacity of truck ?t, i.e., the amount of sugar that can still be loaded onto it.
        (truck-remaining-capacity ?t - truck)

        ; Represents the amount of sugar of brand ?b currently loaded in truck ?t.
        (truck-load ?b - brand ?t - truck)
        
        ; Represents the amount of sugar of brand ?b currently stored at location ?m.
        (storage-stock ?m - location ?b - brand)
	
        ; Represents the cumulative distance traveled by vehicles in the supply chain, used for cost estimation.
        (total-distance)
  
        ; Specifies the distance between two locations ?l1 and ?l2, which can be used to calculate transportation costs.
        (distance ?l1 ?l2 - location)

        ; Tracks the quantity of raw cane available at mill ?m.
        (has-resource ?r - raw-cane ?m - mill)
        
        ; Defines the maximum number of production process changes allowed for mill ?m.
        (max-changing ?m - mill)

		; (inventory-cost) deleted we can add it. 
		
		; Represents the maintenance requirement level for crane ?c.
        ; A value of 0 means no maintenance is needed, while a positive value indicates the degree of maintenance required.
        (need-maintenance ?c - crane)

  
        ; Represents the cumulative time spent on maintenance across all crane operations.
        (total-maintenance-time)

        ; This value indicate the time spent on maintenance a crane.
        (maintenance-time ?c - crane)
	
		; Represents the cumulative cost incurred from handling operations,
        ; such as loading and unloading sugar. This cost reflects the labor and equipment expenses involved in these processes.
        (handling-cost)
	
	)







)



