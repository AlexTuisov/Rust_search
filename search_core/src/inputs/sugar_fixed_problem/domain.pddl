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
	(:requirements :typing :fluents :equality)
	(:types 
        ; brand: different types or varieties of sugar.
        ; raw-cane: the unrefined material used to produce sugar.
        ; mill: production facility that processes raw cane into sugar.
        ; depot: storage or distribution facility.
        ; farm-field: field on a farm for harvesting raw cane.
        ; truck: vehicle used for transporting sugar.
        ; crane: mechanical loader used for handling sugar.

		 sugar location loader - object
		 brand raw-cane          - sugar
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

        ; Indicates that the farm-field ?f is located near or is associated with mill ?m.
        (farmfield-near-mill ?f -farm-field ?m -milis)

        ; Indicates that raw cane of type ?r can be harvested from the specified farm field ?f.
        (farmfield-harvest-cane ?f -farm-field ?r -raw-cane)

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
        (cane-yield ?r -raw-cane ?f - farm-field)
        
        ; Indicates the total amount of cane currently available on a specific farm-field.
        (total-canes ?f - farm-field)

        ; Represents the maximum amount of sugar that the crane can handle in a single operation.
        (capacity ?c - crane)
  
        ; Indicates the remaining capacity of truck ?t, i.e., the amount of sugar that can still be loaded onto it.
        (truck-remaining-capacity ?t - truck)

        ; Represents the amount of sugar of brand ?b currently loaded in truck ?t.
        (truck-load ?b - brand ?t - truck)
        
        ; Represents the amount of sugar of brand ?b currently stored at location ?m.
        (in-storage ?m - location ?b - brand)
	
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

    ; Action: produce_sugar
    ; Produces one unit of sugar.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - At least one unit of raw cane is available.
    ; Effects:
    ;   - Increases storage by 1 unit.
    ;   - Decreases raw cane by 1 unit.
    ;   - Marks the mill as unavailable (busy) for the next production cycle.
    ;   - Increases the mill's total cost by the process cost.
    (:action produce_sugar
		:parameters (?r - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
				(available ?m)
				(>(has-resource ?r ?m)0)
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)1)
				(decrease (has-resource ?r ?m)1)
				(not(available ?m))
				(increase (mills-total-cost)(process-cost ?m))
		     	     )
	)

    ; Action: produce_sugar_max
    ; Produces the maximum batch of sugar when enough raw cane is available.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The available raw cane is at least equal to the maximum batch size (max-produce).
    ; Effects:
    ;   - Increases storage by max-produce units.
    ;   - Decreases raw cane by max-produce units.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 5 times the process cost.
	(:action produce_sugar_max
		:parameters (?r - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
				(available ?m)
			    (>=(has-resource ?r ?m)(max-produce ?m))
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(max-produce ?m))
				(decrease (has-resource ?r ?m)(max-produce ?m))
				(not(available ?m))
				(increase (mills-total-cost) (* 5 (process-cost ?m)))
		     	     )
	)

    ; Action: produce_sugar_by_resource
    ; Produces sugar using all the available raw cane when it is less than the maximum batch size.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The available raw cane is less than max-produce.
    ; Effects:
    ;   - Increases storage by the exact amount of raw cane available.
    ;   - Sets the raw cane resource at the mill to zero.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 4 times the process cost.
    (:action produce_sugar_by_resource
		:parameters (?r - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
				(available ?m)
			    (< (has-resource ?r ?m)(max-produce ?m))
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(has-resource ?r ?m))
				(assign (has-resource ?r ?m)0)
				(not(available ?m))
				(increase (mills-total-cost) (* 4 (process-cost ?m)))
		     	     )
	)

    ; Action: setting-machine
    ; This action resets the mill to an available state after production.
    ; Preconditions:
    ;   - The mill is not currently available (it is busy).
    ; Effects:
    ;   - Makes the mill available for the next production action.
    (:action setting-machine
		:parameters (?m - mill)
		:precondition (and
				 (not (available ?m))
			      )
		:effect	      (and
				 (available ?m)
			      )
	)


    ; Action: switch-production-process
    ; This action allows a mill to change its production process from one sugar brand to another.
    ; Preconditions:
    ;   - The mill's current production process is set to produce brand ?b1.
    ;   - The mill is capable of producing brand ?b2.
    ;   - A process change from ?b1 to ?b2 is allowed.
    ;   - There is at least one production process change available (max-changing > 0).
    ; Effects:
    ;   - Updates the mill’s production process to brand ?b2.
    ;   - Removes the current production process for brand ?b1.
    ;   - Decreases the available production process changes by 1.
    (:action switch-production-process
        :parameters (?m - mill ?b1 - brand ?b2 - brand)
        :precondition (and
            (current-brand-process ?m ?b1)
            (can-produce ?m ?b2)
            (change-brand-process ?b1 ?b2)
            (> (max-changing ?m) 0)
        )
        :effect (and
            (current-brand-process ?m ?b2)
            (not (current-brand-process ?m ?b1))
            (decrease (max-changing ?m) 1)
        )
    )

    ; Action: order-sugar-cane
    ; This action is used when a mill has run out of raw cane and needs to place an order to acquire more.
    ; Preconditions:
    ;   - The quantity of raw cane available at the mill is exactly 0.
    ; Effects:
    ;   - Places an order for raw cane at the mill.
    (:action order-sugar-cane
        :parameters (?r - raw-cane ?m - mill)
        :precondition (and
            (= (has-resource ?r ?m) 0)   
        )
        :effect (and
            (place-order ?r ?m)         
        )
    )



    ; Action: harvest-cane-mill
    ; This action represents harvesting raw cane from a farm-field and transferring it directly to a mill.
    ; Preconditions:
    ;   - The farm-field ?f is near the mill ?m.
    ;   - The farm-field ?f is capable of harvesting raw cane of type ?r.
    ;   - The mill ?m has placed an order for raw cane ?r.
    ;   - The total amount of cane available in the field is at least equal to the cane yield for one harvest.
    ; Effects:
    ;   - Decreases the total cane available in the field by the cane yield.
    ;   - Increases the amount of raw cane available at the mill by the cane yield.
    ;   - Fulfills the order by removing the placed order.
    ;   - Increases the mill's total cost proportionally to the amount of cane harvested.
    (:action harvest-cane-mill
        :parameters (?f - farm-field ?r - raw-cane ?m - mill)
        :precondition (and
            (farmfield-near-mill ?f ?m)                   ; The field is near the mill.
            (farmfield-harvest-cane ?f ?r)                 ; The field can harvest raw cane ?r.
            (place-order ?r ?m)                           ; An order for raw cane is placed at the mill.
            (>= (total-canes-field ?f) (cane-yield ?f))     ; The field has at least the required yield available.
        )
        :effect (and
            (decrease (total-canes-field ?f) (cane-yield ?f))   ; Decrease the available cane in the field by the yield.
            (increase (has-resource ?r ?m) (cane-yield ?f))       ; Increase the mill's raw cane resource by the yield.
            (not (place-order ?r ?m))                           ; Remove the order after harvesting.
            (increase (mills-total-cost) (cane-yield ?f))        ; Increase the mill's cost proportional to the yield.
        )
    )

    ; Action: harvest-cane-truck
    ; This action represents harvesting raw cane from a farm-field and loading it directly onto a truck.
    ; Preconditions:
    ;   - The truck ?t is located at the farm-field ?f.
    ;   - An order for raw cane ?r (intended for collection) has been placed (Note: Review the association of this order if needed).
    ;   - The truck has enough remaining capacity to load the cane yield.
    ;   - The farm-field has at least the required cane yield available.
    ; Effects:
    ;   - Decreases the total cane available in the field by the cane yield.
    ;   - Increases the amount of raw cane loaded onto the truck by the cane yield.
    ;   - Fulfills the order by removing it.
    ;   - Increases the handling cost proportionally to the amount of cane harvested.
    (:action harvest-cane-truck
        :parameters (?f - farm-field ?r - raw-cane ?t - truck)
        :precondition (and
            (at-location ?t ?f)                             ; The truck is at the farm-field.
            (>= (truck-remaining-capacity ?t) (cane-yield ?f)) ; The truck has sufficient remaining capacity.
            (>= (total-canes-field ?f) (cane-yield ?f))       ; The field has at least the required cane yield available.
        )
        :effect (and
            (decrease (total-canes-field ?f) (cane-yield ?f)) ; Decrease the field's available cane by the yield.
            (increase (has-resource ?r ?m) (cane-yield ?f))     ; Increase the raw cane resource (Note: ?m is not declared here; might need to be truck-load ?r ?t).
            (not (place-order ?r ?m))                         ; Remove the placed order after harvesting.
            (increase (handle-cost) (cane-yield ?f))          ; Increase the handling cost proportional to the harvested yield.
        )
    )


	
	(:action sugar-cane-mills
		:parameters (?r - raw-cane ?m1 ?m2 - mill)
		:precondition (and
				(place-order ?r ?m1)
				(>(has-resource ?r ?m2)0)
			      )
		:effect	      (and
				(increase (has-resource ?r ?m1)1)
				(decrease (has-resource ?r ?m2)1)
				(not (place-order ?r ?m1))
				(decrease(inventory-cost)1)
			      )
	)








)



