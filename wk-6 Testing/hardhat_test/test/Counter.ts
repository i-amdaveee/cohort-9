import { expect } from "chai";
import { network } from "hardhat";

const { ethers } = await network.create();

describe("Test Counter Contract", function () {
  let counter: any;
  const x = 5;
  
  beforeEach(async function () {
    counter = await ethers.deployContract("Counter", [x]);
  
  });


  describe("Deployment Successfull", () =>{
    it("Should set x in state to the constructor argument", async function () {
      const blockchainX = await counter.x();
      expect(blockchainX).to.equal(x);
    
    });

    it("Should set y in state to the constructor argument + 1", async function () {
      const blockchainY = await counter.y();
      expect(blockchainY).to.equal(x + 1);
      
    });

  })

  describe.only("Incrementing the counter", () =>{
    it("Should increment x by 1", async function () {
      await counter.inc();
      
      const blockchainX = await counter.x();
      
      expect(blockchainX).to.equal(x + 1);
    });

    it("Should emit the Increment event when calling the inc() function", async function () {
      await expect(counter.incBy(7)).to.emit(counter, "Increment").withArgs(7n); 
    });


  });



});

 
