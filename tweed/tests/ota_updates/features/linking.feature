Feature: Interface Binding Resolution
  Scenario: Invoke a bound interface function through the engine
    Given an engine with an pricing interface implementation registered in version 0.1.0
    When I invoke the "compute_price" function of interface "example:pricing:pricing@0.1.0" with the argument 3
    Then the result should be 5

  Scenario: Invoke a linked interface function
    Given an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an order interface implementation registered in version 0.1.0
    When I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    Then the result should be 5

  Scenario: Upgrade a linked interface
    Given an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an order interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    When I upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    Then the result should be 7

  Scenario: Partial upgrade a linked interface
    Given an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an order interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    And an engine with an order interface implementation registered in version 0.1.1
    When I upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I invoke the "create_order" function of interface "example:order:order@0.1.1" with the argument 3
    Then the result should be 5
    When I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    Then the result should be 7

  Scenario: Upgrade a linked interface and collect garbage
    Given an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an order interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    When I upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I run the garbage collector
    And I invoke the "compute_price" function of interface "example:pricing:pricing@0.1.0" with the argument 3
    Then the result should be the error: "interface not found"

  Scenario: Upgrade a linked interface and link older version 
    Given an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an order interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    When I upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I add an order interface implementation registered in version 0.1.1
    And I run the garbage collector
    And I invoke the "compute_price" function of interface "example:pricing:pricing@0.1.0" with the argument 3
    Then the result should be 5
