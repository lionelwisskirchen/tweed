Feature: Step based interruption
  Scenario: Interrupt an invoked interface
    Given an engine with a max step size of 1
    And an engine with an pricing interface implementation registered in version 0.1.0
    When I invoke the "compute_price" function of interface "example:pricing:pricing@0.1.0" with the argument 3
    Then the execution should be interrupted
    When I run 3 steps
    Then the result should be 5
    When I run 1 steps
    Then the execution should be stopped

  Scenario: Upgrade an interface and a dependent interface is running
    Given an engine with a max step size of 1
    And an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    And an engine with an order interface implementation registered in version 0.1.0
    When I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    Then the execution should be interrupted
    When I upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I run 6 steps
    Then the result should be 5
    When I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    And I run 7 steps
    Then the result should be 7

 Scenario: Force upgrade an interface and a dependent interface is running
    Given an engine with a max step size of 1
    And an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    And an engine with an order interface implementation registered in version 0.1.0
    When I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    Then the execution should be interrupted
    When I force upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I run 6 steps
    Then the result should be 7

 Scenario: Upgrade an interface and a dependent interface is running on a safe point
    Given an engine with a max step size of 1
    And the engine has safe points activated
    And an engine with an pricing interface implementation registered in version 0.1.0
    And an engine with an pricing interface implementation registered in version 0.1.1
    And an engine with an order interface implementation using safe points registered in version 0.1.0 
    When I invoke the "create_order" function of interface "example:order:order@0.1.0" with the argument 3
    Then the execution should be interrupted
    When I upgrade the "example:pricing:pricing@0.1.0" interface for the "example:order:order@0.1.0" interface
    And I run 17 steps
    Then the result should be 7
