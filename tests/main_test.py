import sys
import os

# Add test_suites to path
current_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.join(current_dir, "test_suites"))

try:
    import direct_test as subtest
except ImportError:
    # Fallback if tests/test_suites/__init__.py doesn't exist or other issues
    from test_suites import direct_test as subtest

def main():
    print("Starting Main Test Suite...")
    
    results = {}
    
    print("\n--- Running Direct Outbound Subtest ---")
    results["direct_test"] = subtest.main()
    
    print("\n--- Test Suite Summary ---")
    all_passed = True
    for test_name, success in results.items():
        status = "PASSED" if success else "FAILED"
        print(f"{test_name}: {status}")
        if not success:
            all_passed = False
            
    if all_passed:
        print("\nALL SUBTESTS PASSED")
        sys.exit(0)
    else:
        print("\nSOME SUBTESTS FAILED")
        sys.exit(1)

if __name__ == "__main__":
    main()
