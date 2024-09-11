import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WS.sendRequest(findTestObject('001 SLA QC/OBJ.001.001.Get List SLA QC', [('sla') : GlobalVariable.SLA, ('token') : GlobalVariable.token
            , ('endpoint') : 'settings/sla/qc']))

WS.sendRequest(findTestObject('001 SLA QC/OBJ.001.002.Get SLA Ticket', [('sla') : GlobalVariable.SLA, ('token') : GlobalVariable.token
            , ('endpoint') : 'settings/sla/ticket']))

WS.sendRequest(findTestObject('001 SLA QC/OBJ.001.003.Put SLA QC', [('sla') : GlobalVariable.SLA, ('token') : GlobalVariable.token
            , ('endpoint') : 'settings/sla/qc']))

WS.sendRequest(findTestObject('001 SLA QC/OBJ.001.004.Put SLA Ticket', [('sla') : GlobalVariable.SLA, ('token') : GlobalVariable.token
            , ('endpoint') : 'settings/sla/ticket']))

'Team And Member\r\n'
WS.delay(2)

'Team And Member\r\n'
WS.sendRequest(findTestObject('002 Team Member Team/OBJ.002.001.Get List Teams', [('teams') : GlobalVariable.Member_Team
            , ('token') : GlobalVariable.token, ('endpoint') : '/api/teams']))

'Team And Member\r\n'
WS.sendRequest(findTestObject('002 Team Member Team/OBJ.002.002.Get List Teams Options', [('teams') : GlobalVariable.Member_Team
            , ('token') : GlobalVariable.token, ('endpoint') : 'api/teams/options']))

'Team And Member\r\n'
WS.sendRequest(findTestObject('002 Team Member Team/OBJ.002.003.Get List Teams Detail', [('teams') : GlobalVariable.Member_Team
            , ('token') : GlobalVariable.token, ('endpoint') : '/api/teams/', ('id') : '66beef9f787cdd8139a71515']))

'Team And Member\r\n'
WS.sendRequest(findTestObject('002 Team Member Team/OBJ.002.004.Put Update Account', [('teams') : GlobalVariable.Member_Team
            , ('token') : GlobalVariable.token, ('endpoint') : 'api/teams/', ('id') : '66beef9f787cdd8139a71515/update-account']))

